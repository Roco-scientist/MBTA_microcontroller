extern crate chrono;
extern crate ht16k33;
extern crate rppal;
extern crate std;

use chrono::prelude::*;
use rppal::i2c::I2c;
use std::{collections::HashMap, thread, time};

pub struct ClockDisplay {
    display: ht16k33::HT16K33<I2c>,
}

impl ClockDisplay {
    /// Creates a new ClockDisplay struct
    pub fn new() -> ClockDisplay {
        // create new i2c interface
        let i2c = I2c::new().unwrap();
        let address = 112u8; // actually 0x70 in hexidecimal which goes to 112
        let mut clock = ht16k33::HT16K33::new(i2c, address);
        clock.initialize().unwrap();
        clock.set_display(ht16k33::Display::ON).unwrap();
        clock
            .set_dimming(ht16k33::Dimming::from_u8(7u8).unwrap())
            .unwrap();
        ClockDisplay { display: clock }
    }

    /// Dispalys the minutes:seconds until the next train on the clock display
    pub fn display_time_until(&mut self, train_time: chrono::DateTime<Utc>) -> () {
        let now = chrono::Utc::now();
        let diff = train_time.signed_duration_since(now);
        println!("{:?}", diff);
        self.display_num(0u8, 6u8, true);
        thread::sleep(time::Duration::from_secs(2));
        self.display_num(0u8, 6u8, false);
    }

    /// Turns on the necessary leds for a number at the indicated location
    fn display_num(&mut self, location: u8, number: u8, on: bool) -> () {
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
            self.display.set_led(led_location, on).unwrap();
        }
    }
}
