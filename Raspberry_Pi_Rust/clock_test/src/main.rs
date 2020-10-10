extern crate ht16k33;
extern crate rppal;
extern crate std;

use std::{collections::HashMap, thread, time};

fn main() {
    let number_conversion: HashMap<u8, u8> = [(0, 63), (1, 6), (2, 91), (3, 79), (4, 102)]
        .iter()
        .cloned()
        .collect();
    let i2c = rppal::i2c::I2c::new().unwrap();
    //    let address = 70u8;
    let address = 112u8; // actually 0x70 in hexidecimal which goes to 112
    let mut clock = ht16k33::HT16K33::new(i2c, address);
    let led_location = ht16k33::LedLocation::new(0, *number_conversion.get(&1).unwrap()).unwrap();
    clock.initialize().unwrap();
    clock.set_display(ht16k33::Display::ON).unwrap();
    clock
        .set_dimming(ht16k33::Dimming::from_u8(7u8).unwrap())
        .unwrap();
    clock.set_led(led_location, true).unwrap();
    thread::sleep(time::Duration::from_secs(2));
}
