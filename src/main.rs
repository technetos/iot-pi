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

macro_rules! id {
    ($pin_id:expr) => {
        $pin_id as u8
    };
}

mod led;

fn print_info() {
    let device_info = DeviceInfo::new().unwrap();
    println!(
        "Model: {} (SoC: {})",
        device_info.model(),
        device_info.soc()
    );
}

fn setup() {
    gpio!().set_mode(id!(18), Mode::Alt4); // PWM is 4
    gpio!().set_mode(id!(2), Mode::Input);
    gpio!().set_pullupdown(id!(2), PullUpDown::PullDown);
}

fn main() {
    print_info();
    setup();

    gpio!().set_async_interrupt(id!(2), Trigger::FallingEdge, |level: Level| {
        println!("{:#?}", level);
        if level == Level::Low {
            gpio!().write(id!(4), Level::High);
            sleep(Duration::from_millis(100));
            gpio!().write(id!(4), Level::Low);
        }
    });

    let blinker = led::Blink::new();
    blinker.configure();

    let times = vec![1000, 500, 200, 100];

    loop {
      times.iter().for_each(|time| {
        blinker.execute(*time);
      });
    }
}
