#![no_std]
extern crate chrono;
extern crate cortex_m_rt;
extern crate f3;
extern crate heapless;
extern crate ht16k33;
extern crate panic_halt;
// extern crate stm32f3xx_hal;

use f3::hal::{
    gpio::{
        gpiob::{PB6, PB7},
        AF4,
    },
    i2c::I2c,
    prelude::*,
    stm32f30x::{self, I2C1},
};
use heapless::consts::*;
use heapless::Vec;
// use stm32f3xx_hal::{i2c, prelude::*, stm32};

/// A struct to hold the display along with the digits for each location
pub struct ClockDisplay {
    display: ht16k33::HT16K33<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>,
    number_leds: Vec<Vec<u8, U7>, U10>,
    minutes_ten: Option<u8>,
    minutes_single: Option<u8>,
    seconds_ten: Option<u8>,
    seconds_single: Option<u8>,
}

// Functions to initialize and change clock display
impl ClockDisplay {
    /// Creates a new ClockDisplay struct
    pub fn new() -> ClockDisplay {
        let dp = stm32f30x::Peripherals::take().unwrap();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

        let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
        let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

        let i2c_master = I2c::i2c1(dp.I2C1, (scl, sda), 400_000.hz(), clocks, &mut rcc.apb1);
        // connect to the I2C address found within raspberry pi.  This is converted to decimal
        // actually 0x70 in hexidecimal which goes to 112
        let address = 112u8;
        // connect the ht16k33 clock chip to i2c connection on the address
        let mut clock = ht16k33::HT16K33::new(i2c_master, address);
        clock.initialize().unwrap();
        // turn clock display on.  Would not work otherwise
        clock.set_display(ht16k33::Display::ON).unwrap();
        // set the dimming of the display.  This can be added to new function later
        clock
            .set_dimming(ht16k33::Dimming::from_u8(7u8).unwrap())
            .unwrap();
        // Below are the led numbers for each number within the clock
        //   _   0
        //  |_|  5, 6, 1
        //  |_|  4, 3, 2
        //
        //  Create a hashmap below with each number and the corresponding leds that need to be
        //  turned on
        let number_leds_hash = Vec::<Vec<u8, U7>, U10>::from_slice(&[
            Vec::from_slice(&[0u8, 1u8, 2u8, 3u8, 4u8, 5u8]).unwrap(),
            Vec::from_slice(&[1u8, 2u8]).unwrap(),
            Vec::from_slice(&[0u8, 1u8, 3u8, 4u8, 6u8]).unwrap(),
            Vec::from_slice(&[0u8, 1u8, 2u8, 3u8, 6u8]).unwrap(),
            Vec::from_slice(&[1u8, 2u8, 5u8, 6u8]).unwrap(),
            Vec::from_slice(&[0u8, 2u8, 3u8, 5u8, 6u8]).unwrap(),
            Vec::from_slice(&[0u8, 2u8, 3u8, 4u8, 5u8, 6u8]).unwrap(),
            Vec::from_slice(&[0u8, 1u8, 2u8]).unwrap(),
            Vec::from_slice(&[0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8]).unwrap(),
            Vec::from_slice(&[0u8, 1u8, 2u8, 3u8, 5u8, 6u8]).unwrap(),
        ])
        .unwrap();
        // return ClockDisplay struct with empty digits to be filled later
        ClockDisplay {
            display: clock,
            number_leds: number_leds_hash,
            minutes_ten: None,
            minutes_single: None,
            seconds_ten: None,
            seconds_single: None,
        }
    }

    /// Dispalys the minutes:seconds until the next train on the clock display
    pub fn display_time_until(
        &mut self,
        train_times: &Vec<chrono::NaiveTime, U10>,
        minimum_display_min: &i64,
    ) -> () {
        // get now time in UTC
        let now = chrono::NaiveTime::from_hms(12, 27, 30);
        // get the difference between now and the train time
        let mut diff = train_times[0].signed_duration_since(now);
        // if difference is less than minumum display, use next train
        if diff.num_minutes() < *minimum_display_min {
            if train_times.len() > 1usize {
                diff = train_times[1].signed_duration_since(now)
            } else {
                // if there is not a next train, clear display and end
                self.clear_display();
                return ();
            }
        }
        // separate out minutes and seconds for the display
        let minutes = diff.num_minutes();
        // Seconds as the remainder after minutes are removed
        let seconds = diff.num_seconds() % 60i64;
        // Clock display only has two digits for minutes, so minutes need to be below 100
        if *minimum_display_min < minutes && minutes < 100i64 {
            // find all of the new digits for displaying difference
            // first digit, which is the tens minutes
            let first = (minutes as u8) / 10u8;
            // second digit, which is the single minutes
            let second = (minutes as u8) % 10u8;
            // third digit, which is the seconds ten
            let third = (seconds as u8) / 10u8;
            // fourth digit, which is the seconds single
            let fourth = (seconds as u8) % 10u8;
            // if current display has no values, then display all of the new values
            if [
                self.minutes_ten,
                self.minutes_single,
                self.seconds_ten,
                self.seconds_single,
            ]
            .iter()
            .any(|digit| digit.is_none())
            {
                self.minutes_ten = Some(first);
                self.minutes_single = Some(second);
                self.seconds_ten = Some(third);
                self.seconds_single = Some(fourth);
                self.display_nums();
            } else {
                // else change only the values that have changed
                if Some(first) != self.minutes_ten {
                    self.change_number(0, &first);
                    self.minutes_ten = Some(first);
                }
                if Some(second) != self.minutes_single {
                    self.change_number(2, &second);
                    self.minutes_single = Some(second);
                }
                if Some(third) != self.seconds_ten {
                    self.change_number(6, &third);
                    self.seconds_ten = Some(third);
                }
                if Some(fourth) != self.seconds_single {
                    self.change_number(8, &fourth);
                    self.seconds_single = Some(fourth);
                }
            }
        } else {
            // if minutes is greater than 100 clear dispaly and set all values to none
            self.clear_display();
        }
    }

    /// Clears clock display
    pub fn clear_display(&mut self) -> () {
        //set all values to None
        self.minutes_ten = None;
        self.minutes_single = None;
        self.seconds_ten = None;
        self.seconds_single = None;
        // clear the display buffer then push to clock to create a clear clock
        self.display.clear_display_buffer();
        self.display.write_display_buffer().unwrap();
    }

    /// Turns on all numbers
    fn display_nums(&mut self) -> () {
        // Retrieve a vec! of leds that need to be turned on for the numbers
        // Then turn them on
        let leds = &self.number_leds[self.minutes_ten.unwrap() as usize];
        self.switch_leds(&leds.clone(), 0, true);
        let leds = &self.number_leds[self.minutes_single.unwrap() as usize];
        self.switch_leds(&leds.clone(), 2, true);
        let leds = &self.number_leds[self.seconds_ten.unwrap() as usize];
        self.switch_leds(&leds.clone(), 6, true);
        let leds = &self.number_leds[self.seconds_single.unwrap() as usize];
        self.switch_leds(&leds.clone(), 8, true);
        self.display_colon(true);
    }

    /// Turns on/off the necessary leds for a number at the indicated location
    fn switch_leds(&mut self, leds: &Vec<u8, U7>, location: u8, on: bool) -> () {
        // Turn on/off each led
        for led in leds {
            let led_location = ht16k33::LedLocation::new(location, *led).unwrap();
            self.display.set_led(led_location, on).unwrap();
        }
    }

    /// Turns on/off the colon between the digits for the clock
    fn display_colon(&mut self, on: bool) -> () {
        let leds = [0u8, 1u8];
        // Turn on/off each led
        for led in &leds {
            // colon is located at location 4, with leds 1,2
            let led_location = ht16k33::LedLocation::new(4u8, *led).unwrap();
            self.display.set_led(led_location, on).unwrap();
        }
    }

    fn change_number(&mut self, location: u8, new_number: &u8) {
        // determine which struct digit to pull based on led location
        let old_number_option = match location {
            0u8 => self.minutes_ten,
            2u8 => self.minutes_single,
            6u8 => self.seconds_ten,
            8u8 => self.seconds_single,
            _ => panic!("location not recognized"),
        };
        // get the leds for the new number
        let new_leds = &self.number_leds[*new_number as usize];
        if let Some(old_number) = old_number_option {
            // get the leds for th old number
            let old_leds = &self.number_leds[old_number as usize];
            // get what leds are in the old number and not the new to then be able to turn off
            let leds_off = old_leds
                .iter()
                .filter_map(|led| {
                    if !new_leds.contains(led) {
                        Some(led.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<u8, U7>>();
            // get what leds are in the new number but no tht eold to then be able to turn on
            // these two are used so that instead of turning all off then new on, only switching the
            // necessary leds
            let leds_on = new_leds
                .iter()
                .filter_map(|led| {
                    if !old_leds.contains(led) {
                        Some(led.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<u8, U7>>();
            // turn off leds
            self.switch_leds(&leds_off, location, false);
            // turn on leds
            self.switch_leds(&leds_on, location, true)
        } else {
            self.switch_leds(&new_leds.clone(), location, true)
        }
    }
}
