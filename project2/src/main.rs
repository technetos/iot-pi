use std::{fs::File, io, error::Error};
use coap::{CoAPServer, CoAPResponse, CoAPRequest, Method};

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct Temperature {
    pub fahrenheit: f64,
    pub celsius: f64,
}

// Read data from the temperature sensor and return the parsed temperature.
fn read_temperature<'r>() -> Result<Temperature, &'r str> {
    let file = File::open("/sys/bus/w1/devices/28-00000755f9f2/w1_slave")
        .map_err(|_| "error reading temperature")?;

    println!("{:#?}", file);

    Ok(Temperature {
        fahrenheit: 0.0,
        celsius: 0.0,
    })
}


fn request_handler(request: CoAPRequest) -> Option<CoAPResponse> {
    use coap::message::IsMessage;

    match request.get_method() {
        &Method::Get if request.get_path() == "/sensors/temp" => {
            println!(" => GET {}", request.get_path());
            let packet = request.get_message();
            

            return None
        }
        _ => return None
    }
 
    // Return the auto-generated response
   request.response
}

fn main() {
    let port = 8080;
    println!("Listening for connections on port {}", port);

}
