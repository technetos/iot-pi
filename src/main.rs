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
    static ref TIMER: Mutex<PauseLoop> = Mutex::new(PauseLoop::new());
}

macro_rules! gpio {
    () => {
        GPIO.lock().unwrap()
    };
}

macro_rules! timer {
  () => {
    TIMER.lock().unwrap()
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

struct PauseLoop {
  times: Vec<u64>,
  index: usize,
  paused: bool,
}

impl PauseLoop {
  pub fn new() -> Self {
    Self {
      times: vec![5000, 3000, 1500, 1000, 700],
      index: 0,
      paused: false,
    }
  }

  pub fn is_paused(&mut self) -> bool {
    self.paused
  }

  pub fn pause(&mut self) {
    self.paused = true;
  }

  pub fn start(&mut self) {
    self.paused = false;
  }

  pub fn get_time(&mut self) -> Duration {
    let old_index = self.index;

    if self.paused {
      return t!(self.times[old_index]);
    }

    if old_index == 4 {
      self.index = 0;
    } else {
      self.index = old_index + 1;
    }

    t!(self.times[old_index])
  }
}

fn main() {
  print_info();
  setup();

  gpio!().set_async_interrupt(pin!(2), Trigger::FallingEdge, |level: Level| {
    println!("{:#?}", level);
    if timer!().is_paused() {
      timer!().start();
    } else {
      timer!().pause();
    }
  });

  loop {
    let time = timer!().get_time();

    gpio!().write(pin!(17), Level::High);
    sleep(time);
    gpio!().write(pin!(17), Level::Low);
    sleep(time);
  }
}
