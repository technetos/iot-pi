use rppal::{
    gpio::{Gpio, Level, Mode, PullUpDown, Trigger},
    pwm::{Channel, Polarity, Pwm},
    system::DeviceInfo,
};

use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

pub struct Blink {
    pwm: Pwm,
}

impl Blink {
    pub fn new() -> Self {
        Self {
          pwm: Pwm::new(Channel::Pwm0).expect("failed on pwm"),
        }
    }

    pub fn configure(&self) {
      self.pwm.set_period(Duration::from_millis(1000));
      self.pwm.set_duty_cycle(Duration::from_millis(500));
      self.pwm.enable();

      println!("{:#?}", self.pwm.duty_cycle());
    }

    pub fn execute(&self, time: u64) {
        sleep(Duration::from_millis(time));
    }

}
