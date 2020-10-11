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
        // connect to the I2C address found within raspberry pi.  This is converted to decimal
        let address = 112u8; // actually 0x70 in hexidecimal which goes to 112
                             // connect the ht16k33 clock chip to i2c connection on the address
        let mut clock = ht16k33::HT16K33::new(i2c, address);
        clock.initialize().unwrap();
        // turn clock display on.  Would not work otherwise
        clock.set_display(ht16k33::Display::ON).unwrap();
        // set the dimming of the display.  This can be added to new function later
        clock
            .set_dimming(ht16k33::Dimming::from_u8(7u8).unwrap())
            .unwrap();
        // return ClockDisplay struct
        ClockDisplay { display: clock }
    }

    /// Dispalys the minutes:seconds until the next train on the clock display
    pub fn display_time_until(&mut self, train_time: chrono::DateTime<Utc>) -> () {
        // get now time in UTC
        let now = chrono::Utc::now();
        // get the difference between now and the train time
        let diff = train_time.signed_duration_since(now);
        // separate out minutes and seconds for the display
        let minutes = diff.num_minutes() as u8;
        let seconds = diff.num_seconds() as u8;
        if minutes < 100u8 {
            let first = minutes / 10u8;
            let second = minutes % 10u8;
            let third = seconds / 10u8;
            let fourth = seconds % 10u8;
            self.display_num(0u8, first, true);
            self.display_num(2u8, second, true);
            self.display_colon(true);
            self.display_num(6u8, third, true);
            self.display_num(8u8, fourth, true);
        }
        println!("{:?}:{:?}", minutes, seconds);
        println!("{:?}:{:?}", diff);
        thread::sleep(time::Duration::from_secs(2));
        self.display.clear_display_buffer();
        self.display.write_display_buffer().unwrap();
    }

    /// Turns on the necessary leds for a number at the indicated location
    fn display_num(&mut self, location: u8, number: u8, on: bool) -> () {
        // Below are the led numbers for each number within the clock
        //   _   0
        //  |_|  5, 6, 1
        //  |_|  4, 3, 2
        //
        //  Create a hashmap below with each number and the corresponding leds that need to be
        //  turned on
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

        // Retrieve a vec! of leds that need to be turned on for the numbers
        let leds = number_leds.get(&number).unwrap();
        // Turn on/off each led
        for led in leds {
            let led_location = ht16k33::LedLocation::new(location, *led).unwrap();
            self.display.set_led(led_location, on).unwrap();
        }
    }

    fn display_colon(&mut self, on: bool) -> () {
        let leds = [0u8, 1u8];
        // Turn on/off each led
        for led in leds {
            let led_location = ht16k33::LedLocation::new(4u8, *led).unwrap();
            self.display.set_led(led_location, on).unwrap();
        }
    }
}
