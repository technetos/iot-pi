use coap::{CoAPClient, CoAPResponse};

fn main() {
    let url = "coap://10.10.3.99:5683/sensors/temp";
    println!("Client request: {}", url);

    loop {
        match CoAPClient::get(url) {
            Ok(response) => {
                println!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());
                break;
            }
            Err(e) => println!("Request error: {:?}", e),
        }
    }
}
