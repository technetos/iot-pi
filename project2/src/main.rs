use coap::{CoAPRequest, CoAPResponse, CoAPServer, Method};
use std::{
    fs::File,
    io::{self, Read},
};

use serde_derive::{Deserialize, Serialize};
use serde_json::{json, to_vec};

static ADDRESS: &str = "192.168.1.12:5683";

#[derive(Serialize, Deserialize, Debug)]
struct Temperature {
    pub fahrenheit: f64,
    pub celsius: f64,
}

// Read data from the temperature sensor and return the parsed temperature.
fn read_temperature<'r>() -> Result<Temperature, &'r str> {
    let mut contents = String::new();

    File::open("/sys/bus/w1/devices/28-00000755f9f2/w1_slave")
        .map_err(|_| "error opening temperature file")?
        .read_to_string(&mut contents)
        .map_err(|_| "error reading temperature")?;

    let temp_str = contents.split("t=").collect::<Vec<&str>>();
    let raw_temp = temp_str.last().unwrap().trim_end_matches("\n");

    let celsius = raw_temp
        .parse::<f64>()
        .map_err(|_| "invalid temperature data")?
        / 1000.0;

    let fahrenheit = (celsius * 1.8) + 32.0;

    println!("f: {:.2} c: {:.2}", fahrenheit, celsius);

    Ok(Temperature {
        fahrenheit,
        celsius,
    })
}

fn request_handler(request: CoAPRequest) -> Option<CoAPResponse> {
    use coap::message::IsMessage;

    match request.get_method() {
        &Method::Get if request.get_path() == "sensors/temp" => {
            println!(" => GET {}", request.get_path());

            // Convert the payload into json and then into a vector of
            // bytes to be sent over the wire.
            let payload = to_vec(&json!(read_temperature().unwrap())).unwrap();

            request.response.map(|mut r| {
                r.set_payload(payload);
                r
            })
        }
        _ => return None,
    }
}

fn main() {
    let mut server = CoAPServer::new(ADDRESS).unwrap();
    server.handle(request_handler).unwrap();

    println!("Server up on {}", ADDRESS);
    println!("Press any key to stop...");
    io::stdin().read_line(&mut String::new()).unwrap();

    println!("Server shutdown");
}
