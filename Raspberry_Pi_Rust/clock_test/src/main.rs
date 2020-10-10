extern crate ht16k33;
extern crate rppal;
extern crate std;

use std::{thread, time};

fn main() {
    let i2c = rppal::i2c::I2c::new().unwrap();
    let address = 112u8; // actually 0x70 in hexidecimal which goes to 112
    let mut clock = ht16k33::HT16K33::new(i2c, address);
    clock.initialize().unwrap();
    clock.set_display(ht16k33::Display::ON).unwrap();
    clock
        .set_dimming(ht16k33::Dimming::from_u8(7u8).unwrap())
        .unwrap();
    let led_location = ht16k33::LedLocation::new(0, 0).unwrap();
    clock.set_led(led_location, true).unwrap();
    thread::sleep(time::Duration::from_secs(2));
    clock.set_led(led_location, false).unwrap();
}

