#![deny(unsafe_code)]
#![deny(warning)]
#![no_main]
#![no_std]
#![feature(used)]
#![feature(const_fn)]

extern crate chrono;
extern crate cortex_m;
extern crate cortex_m_rt;
#[macro_use]
extern crate rtic;
extern crate f3;
extern crate heapless;
extern crate panic_semihosting;
extern crate stm32f3xx_hal;

use chrono::{prelude::*, NaiveTime};
use core::time::Duration;
use forest_hills_departure;
use heapless::{consts::*, Vec};
use panic_semihosting as _;
use rtic::app;
use rtic::{
    app,
    cyccnt::{Instant, U32Ext},
};

const PERIOD_DISPLAY: u32 = 8_000;
const PERIOD_TRAINS: u32 = 8_000_000;

#[app(device = f3::hal::stm32f30x, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        time_now: NaiveTime,
        train_times: Option<Vec<chrono::NaiveTime, U10>>,
    }
    #[init(schedule = [get_trains, display_times])]
    fn init(cx: init::Context) -> init::LateResources {
        let now_plus_5 = chrono::NaiveTime::from_hms(12, 32, 30);
        let mut train_vec: Vec<chrono::NaiveTime, U10> = Vec::new();
        train_vec.push(now_plus_5);
        init::LateResources {
            time_now: NaiveTime::from_hms(12, 27, 30),
            train_times: Some(train_vec),
        };
        cx.core.DCB.enable_trace();
        cx.schedule
            .display_times(cx.start + PERIOD_DISPLAY)
            .unwrap();
        cx.schedule.get_trains(cx.start + PERIOD_TRAINS).unwrap();
    }

    #[task(schedule = [get_trains], resources = [time_now, train_times])]
    fn get_trains(cx: get_trains::Context) {
        cx.resources.train_times.lock(|train_times| {
            if cx.resources.time_now > train_times.unwrap()[0] {
                let now_plus_5 = cx.resources.time_now + chrono::Duration::minutes(5);
                let mut train_vec: Vec<chrono::NaiveTime, U10> = Vec::new();
                train_vec.push(now_plus_5);
                *train_times = Some(train_vec);
            }
        });
        cx.schedule
            .get_trains(cx.scheduled + PERIOD_TRAINS)
            .unwrap();
    }

    #[task(schedule = [display_times], priority = 2, resources = [time_now, train_times])]
    fn display_times(cx: display_times::Context) {
        let minimum_display_min = 5i64;
        // create a new clock struct, this initializes the display
        let mut clock = forest_hills_departure::ht16k33_clock::ClockDisplay::new();
        // create a new screen struct, this initializes the display
        let mut screen = forest_hills_departure::ssd1306_screen::ScreenDisplay::new();
        *cx.resources.time_now += chrono::Duration::milliseconds(250);
        // if there are some train times, display on clock and screen
        if let Some(train_times) = cx.resources.train_times {
            screen.display_trains(&train_times);
            clock.display_time_until(&train_times, &minimum_display_min);
        } else {
            // if there are no train times, clear both displays
            screen.clear_display(true);
            clock.clear_display();
        }
        cx.schedule
            .display_times(cx.scheduled + PERIOD_DISPLAY)
            .unwrap();
    }
    extern "C" {
        fn SSI0();
        fn QEI0();
    }
};
