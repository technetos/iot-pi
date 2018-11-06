use std::fmt;
use coap::{CoAPClient, CoAPResponse};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, to_vec};

#[derive(Serialize, Deserialize, Debug)]
struct Temperature {
    pub fahrenheit: f64,
    pub celsius: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\tFahrenheit: {:.2}\n\tCelsius {:.2}", self.fahrenheit, self.celsius)
    }
}

fn main() {
    let url = "coap://192.168.1.12:5683/sensors/temp";
    println!("Client request: {}", url);

    loop {
        match CoAPClient::get(url) {
            Ok(response) => {
                let payload: Temperature = serde_json::from_slice(&response.message.payload[..]).unwrap();
                println!("Current Temperature\n{}", payload);
                break;
            }
            Err(e) => println!("No Data"),
        }
    }
}
