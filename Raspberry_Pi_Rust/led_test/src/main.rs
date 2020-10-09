extern crate rppal;
extern crate std;

use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(37).unwrap().into_output();

    pin.set_high();
    thread::sleep(Duration::from_secs(1));
    pin.set_low();
}
