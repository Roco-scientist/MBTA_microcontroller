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
extern crate heapless;
extern crate stm32f3xx_hal;
extern crate panic_semihosting;
extern crate f3;

use chrono::{prelude::*, DateTime};
use core::{cell::Cell, time::Duration};
use forest_hills_departure;
use heapless::{consts::*, Vec};
use rtfm::app;
use f3::hal::stm32f30x::delay;

static TRAIN_TIMES: Option<Vec<chrono::NaiveTime, U10>>;
static NOW: Resource<Cell<chrono::NaiveTime>, C1>;
static mut DELAY: delay::Delay;

#[app(device = f3::hal::stm32f30x)]
const APP: () = {
    #[init(resources = [TRAIN_TIMES, DELAY, NOW])]
    fn init() {
        resources.NOW.lock(|now|{
            *now = chrono::NaiveTime::from_hms(12, 27, 30);
        });
        let now_plus_5 = chrono::NaiveTime::from_hms(12, 32, 30);
        let mut train_vec: Vec<chrono::NaiveTime, U10> = Vec::new();
        train_vec.push(now_plus_5);
        resources.TRAIN_TIMES.lock(|train_times|{
            *train_times = Some(train_vec)
        });
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = stm32f3xx_hal::Peripherals::take().unwrap();
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        resources.DELAY.lock(|delay|{
            *delay = delay::Delay::new(cp.SYST, clocks);
        });
    }

    #[interrupt(priority = 1, resources = [TRAIN_TIMES, NOW, DELAY])]
    fn get_trains() {
        loop {
            resources.DELAY.delay_ms(60_000u16);
            // let new_train_times = forest_hills_departure::train_time::train_times()
            //     .unwrap_or_else(|err| panic!("{:?}", err));
            resources.TRAIN_TIMES.lock(|train_times|{
                if resources.NOW > train_times[0] {
                    now_plus_5 = resources.NOW + Duration::from_secs(300u64);
                    let mut train_vec: Vec<chrono::NaiveTime, U10> = Vec::new();
                    train_vec.push(now_plus_5);
                    *train_times = Some(train_vec);
                }
            });
        }
    }

    #[interrupt(priority = 2, resources = [TRAIN_TIMES, NOW, DELAY])]
    fn display_times() {
        let minimum_display_min = 5i64;
        // create a new clock struct, this initializes the display
        let mut clock = forest_hills_departure::ht16k33_clock::ClockDisplay::new();
        // create a new screen struct, this initializes the display
        let mut screen = forest_hills_departure::ssd1306_screen::ScreenDisplay::new();
        // clone the train_times to pass into thread
        loop {
            resources.DELAY.delay_ms(250u8);
            *resources.NOW = NOW + Duration::from_ms(250u8)
            // if there are some train times, display on clock and screen
            if let Some(train_times) = resources.TRAIN_TIMES {
                screen.display_trains(&train_times);
                clock.display_time_until(&train_times, &minimum_display_min);
            } else {
                // if there are no train times, clear both displays
                screen.clear_display(true);
                clock.clear_display();
            }
        }
    }

};
// setup multitasking on microcontroller
//tasks!(stm32f3xx_hal,
//get_trains: Task{
//    interrupt: Tim6,
//    priority: P0,
//    enabled: true
//},
//display_times: Task{
//    interrupt: Time6,
//    priority: P1,
//    enabled: true
//});


//fn idle(priority: P0, threshold: T0) {
//    loop {
//        cortex_m_rtfm::wfi()
//    }
//}


