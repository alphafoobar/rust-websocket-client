mod date_helper;

use std::env;
use std::str;
use std::string::String;

use chrono::{DateTime, TimeZone, Utc};
use serde_json::Value;
use websocket::ws::dataframe::DataFrame;
use websocket::ClientBuilder;

fn main() {
    let key = "WEBSOCKET_ADDRESS";
    match env::var(key) {
        Ok(address) => {
            let mut client = ClientBuilder::new(address.as_str())
                .unwrap()
                .connect(None)
                .unwrap();
            let mut previous = Utc::now();
            let mut counter: u128 = 0;
            println!("{:?}: Connected", Utc::now());

            for message in client.incoming_messages() {
                // Skip some messages.
                let text = message.unwrap().take_payload();
                let s = str::from_utf8(&text.as_slice()).unwrap();
                let v: Value = serde_json::from_str(&s).unwrap();
                let event_type = v["event"].as_str();
                let service_name = v["service"].as_str();

                if event_type.is_some() && event_type.unwrap().eq("DISCONNECT") {
                    println!("{:?}: [{}] -> DISCONNECT {}", Utc::now(), counter, s);
                } else if event_type.is_some() && event_type.unwrap().eq("WELCOME") {
                    println!("{:?}: [{}] -> WELCOME {}", Utc::now(), counter, s);
                } else {
                    let index_name = v["indexName"].as_str();
                    let rate = v["rate"].as_str();
                    let time = v["timestamp"].as_str();
                    let constituents = v["constituents"].as_array();

                    if index_name.is_some()
                        && rate.is_some()
                        && time.is_some()
                    {
                        if constituents.is_some()
                        {
                            let index_name_string = index_name.unwrap();
                            let result = DateTime::parse_from_rfc3339(time.unwrap());
                            let now = Utc::now();
                            let now_nanos = now.timestamp_nanos();
                            let lag =
                                ((now_nanos - result.unwrap().timestamp_nanos()) as f64) / 1000000.0;
                            let delay = ((now_nanos - previous.timestamp_nanos()) as f64) / 1000000.0;
                            let prop_delay = ((now_nanos
                                - propagation_delay(constituents.unwrap().to_vec()).timestamp_nanos())
                                as f64)
                                / 1000000.0;
                            let algo = algorithm_name(constituents.unwrap().to_vec());
                            println!(
                                "[{}] {:?}:{:?} [{}] -> name={}, rate={}, algo=\"{}\", inputs={:?}, propagation-delay={}, lag={}, delay={}",
                                service_name_or(service_name),
                                now,
                                result.unwrap(),
                                counter,
                                index_name_string,
                                rate.unwrap(),
                                algo,
                                map(constituents.unwrap().to_vec()),
                                prop_delay,
                                lag,
                                delay
                            );
                            previous = now;

                            counter = counter + 1;
                        } else {
                            let index_name_string = index_name.unwrap();
                            let result = DateTime::parse_from_rfc3339(time.unwrap());
                            let now = Utc::now();
                            let now_nanos = now.timestamp_nanos();
                            let lag =
                                ((now_nanos - result.unwrap().timestamp_nanos()) as f64) / 1000000.0;
                            let delay = ((now_nanos - previous.timestamp_nanos()) as f64) / 1000000.0;

                            println!(
                                "[{}] {:?}:{:?} [{}] -> name={}, rate={}, lag={}, delay={}",
                                service_name_or(service_name),
                                now,
                                result.unwrap(),
                                counter,
                                index_name_string,
                                rate.unwrap(),
                                lag,
                                delay
                            );
                            previous = now;

                            counter = counter + 1;
                        }
                    }
                }
            }
        }
        Err(error) => println!("Must define variable \"{}\": {}", key, error),
    }
}

fn service_name_or(service_name: Option<&str>) -> &str {
    if service_name.is_some()
    {
        return service_name.unwrap();
    }
    return "?";
}

fn map(vs: Vec<Value>) -> Vec<String> {
    let mut xs = Vec::new();
    for i in vs.iter() {
        let v = i
            .as_object()
            .unwrap()
            .get("midPrice")
            .unwrap()
            .as_str()
            .unwrap();
        xs.push(v.to_string());
    }
    return xs;
}

fn algorithm_name(vs: Vec<Value>) -> String {
    for i in vs.iter() {
        let option = i.as_object().unwrap().get("algorithmName");

        if option.is_some() {
            return option.unwrap().as_str().unwrap().to_string();
        }
    }
    return "trimmed".to_string();
}

fn propagation_delay(vs: Vec<Value>) -> DateTime<Utc> {
    let mut latest: DateTime<Utc> = Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 0);
    for i in vs.iter() {
        let ts = date_helper::parse_datetime(
            i.as_object()
                .unwrap()
                .get("lastUpdatedTimestamp")
                .unwrap()
                .as_str()
                .unwrap(),
        );

        if ts.gt(&latest) {
            latest = ts;
        }
    }
    return latest;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    mod fs_helper;

    #[test]
    fn test_propagation_delay() {
        let string = fs_helper::read_file_from_relative_path("resources/test/constituents.json");
        let expected = date_helper::parse_datetime("2020-03-12T03:37:13.646613Z");
        let vec = serde_json::from_str(string.as_str()).unwrap();

        assert_eq!(propagation_delay(vec), expected);
    }
}
