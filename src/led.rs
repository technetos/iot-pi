use sysfs_gpio::{Direction, Pin};

pub struct Led;

impl Led {
    pub fn new(id: u64) -> Pin {
        let pin = Pin::new(id);
        let _ = pin.export().expect("Failed to export Button pin");
        pin
    }

    pub fn direction(pin: &Pin) {
        pin.set_direction(Direction::Out)
            .expect("Failed to set Led direction");
    }
}
