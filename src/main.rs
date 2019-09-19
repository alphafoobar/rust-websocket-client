use std::env;

use chrono::Utc;
use websocket::ClientBuilder;

fn main() {
    let key = "WEBSOCKET_ADDRESS";
    match env::var(key) {
        Ok(address) => {
            let mut client = ClientBuilder::new(address.as_str())
                .unwrap()
                .connect_secure(None)
                .unwrap();

            let mut counter: u128 = 0;
            for message in client.incoming_messages() {
                // Skip some messages.
                if counter % 10 == 0 {
                    println!("{:?}: -> {:?}", Utc::now(), &message.unwrap());
                }
                counter = counter + 1;
            }
        }
        Err(error) => println!("Must define variable \"{}\": {}", key, error),
    }
}
