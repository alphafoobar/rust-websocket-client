use std::env;
use std::str;

use chrono::{DateTime, Utc};
use serde_json::Value;
use websocket::ClientBuilder;
use websocket::ws::dataframe::DataFrame;

fn main() {
    let key = "WEBSOCKET_ADDRESS";
    match env::var(key) {
        Ok(address) => {
            let mut client = ClientBuilder::new(address.as_str())
                .unwrap()
                .connect_insecure()
//                .connect_secure(None)
                .unwrap();

            let mut counter: u128 = 0;
            for message in client.incoming_messages() {
                // Skip some messages.
                let text = message.unwrap().take_payload();
                let s = str::from_utf8(&text.as_slice()).unwrap();
                let v: Value = serde_json::from_str(&s).unwrap();
                let event_type = v["event"].as_str();
                if event_type.is_some() && event_type.unwrap().eq("DISCONNECT") {
                    println!("{:?}: [{}] -> DISCONNECT {}", Utc::now(), counter, s);
                } else {
                    let rate = v["rate"].as_str();
                    let time = v["timestamp"].as_str();
                    if rate.is_some() && time.is_some() {
                        let result = DateTime::parse_from_rfc3339(time.unwrap());
                        let now = Utc::now();
                        let lag = ((now.timestamp_nanos() - result.unwrap().timestamp_nanos()) as f64) / 1000000.0;
                        println!("{:?}: [{}] -> rate={}, time={}, lag={}", now, counter, rate.unwrap(), time.unwrap(), lag);
                    }
                }
                counter = counter + 1;
            }
        }
        Err(error) => println!("Must define variable \"{}\": {}", key, error),
    }
}
