use rppal::{
    gpio::{Gpio, Level, Mode, PullUpDown, Trigger},
    pwm::{Channel, Polarity, Pwm},
    system::DeviceInfo,
};
use std::process::Command;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GPIO: Mutex<Gpio> = Mutex::new(Gpio::new().unwrap());
}

macro_rules! gpio {
    () => {
        GPIO.lock().unwrap()
    };
}

macro_rules! pin {
    ($pin_id:expr) => {
        $pin_id as u8
    };
}

macro_rules! t {
  ($time:expr) => {
    Duration::from_millis($time)
  };
}

fn print_info() {
    let device_info = DeviceInfo::new().unwrap();
    println!(
        "Model: {} (SoC: {})",
        device_info.model(),
        device_info.soc()
    );
}

fn setup() {
    gpio!().set_mode(pin!(17), Mode::Output); // PWM is 4
    gpio!().set_mode(pin!(2), Mode::Input);
    gpio!().set_pullupdown(pin!(2), PullUpDown::PullDown);
}

fn main() {
    print_info();
    setup();

    gpio!().set_async_interrupt(pin!(2), Trigger::FallingEdge, |level: Level| {
        println!("{:#?}", level);
        if level == Level::Low {
            gpio!().write(pin!(4), Level::High);
            sleep(Duration::from_millis(100));
            gpio!().write(pin!(4), Level::Low);
        }
    });


    let times = vec![3000, 2000, 1000];

    loop {
      for i in &times {
        gpio!().write(pin!(17), Level::High);
        sleep(t!(*i));
        gpio!().write(pin!(17), Level::Low);
        sleep(t!(*i));
      }
    }
}
