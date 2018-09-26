use std::{thread::sleep, time::Duration};
use sysfs_gpio::{Direction, Pin};

const PIN_ID: u64 = 2;

pub fn blink() {
    let led = Pin::new(PIN_ID);
    // do blinking of led via gpio pins
}
