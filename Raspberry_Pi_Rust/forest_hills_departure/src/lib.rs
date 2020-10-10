extern crate chrono;
extern crate ht16k33;
extern crate rppal;
extern crate std;

use rppal::i2c::I2c;
use std::{collections::HashMap, thread, time};

pub fn initialize_display() -> Result<ht16k33::HT16K33<I2c>> {
    let i2c = I2c::new()?;
    let address = 112u8; // actually 0x70 in hexidecimal which goes to 112
    let mut clock = ht16k33::HT16K33::new(i2c, address);
    clock.initialize()?;
    clock.set_display(ht16k33::Display::ON)?;
    clock.set_dimming(ht16k33::Dimming::from_u8(7u8)?)?;
    return Ok(clock);
}

pub fn display_time(train_time: chrono::DateTime, display: &mut ht16k33::HT16K33<I2c>) -> () {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(train_time);
    println!("{:?}", diff);
    display_num(0u8, 6u8, &mut display, true);
    thread::sleep(time::Duration::from_secs(2));
    display_num(0u8, 6u8, &mut display, false);
}

fn display_num(location: u8, number: u8, display: &mut ht16k33::HT16K33<I2c>, on: bool) -> () {
    //   _   0
    //  |_|  5, 6, 1
    //  |_|  4, 3, 2
    let number_leds: HashMap<u8, Vec<u8>> = [
        (0u8, vec![0u8, 1u8, 2u8, 3u8, 4u8, 5u8]),
        (1u8, vec![1u8, 2u8]),
        (2u8, vec![0u8, 1u8, 3u8, 4u8, 6u8]),
        (3u8, vec![0u8, 1u8, 2u8, 3u8, 6u8]),
        (4u8, vec![1u8, 2u8, 5u8, 6u8]),
        (5u8, vec![0u8, 2u8, 3u8, 5u8, 6u8]),
        (6u8, vec![0u8, 2u8, 3u8, 4u8, 5u8, 6u8]),
        (7u8, vec![0u8, 1u8, 2u8]),
        (8u8, vec![0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8]),
        (9u8, vec![0u8, 1u8, 2u8, 3u8, 5u8, 6u8]),
    ]
    .iter()
    .cloned()
    .collect();

    let leds = number_leds.get(&number).unwrap();
    for led in leds {
        let led_location = ht16k33::LedLocation::new(location, *led).unwrap();
        display.set_led(led_location, on).unwrap();
    }
}
