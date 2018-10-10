mod button;
mod led;

use std::process::Command;
use std::{thread::sleep, time::Duration};

use crate::button::Button;
use crate::led::Led;

fn main() {
    let pins = vec![23, 17]
        .into_iter()
        .map(|n| match n {
            23 => Button::new(n),
            _ => Led::new(n),
        })
        .collect::<Vec<_>>();

    // Systemd changes the permissions *at an arbitrary time* after the pins are
    // exported.  To avoid a race condition between systemd changing the
    // permissions and us trying to write to the pin files, we sleep and then
    // change the permissions manually.
    sleep(Duration::from_millis(300));
    Command::new("chmod")
        .args(&["-R", "644", "/sys/class/gpio"])
        .output()
        .expect("Failed to change permissions");

    pins.iter().for_each(|pin| match pin.get_pin_num() {
        23 => Button::direction(pin),
        _ => Led::direction(pin),
    });
}
