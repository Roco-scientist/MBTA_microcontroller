#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(used)]

extern crate chrono;
extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm;
extern crate cortex_m;
extern crate heapless;
extern crate stm32f3xx_hal;

use chrono::{prelude::*, DateTime};
use core::{cell::Cell, time::Duration};
use cortex_m_rtfm::{Resource, TMax, C1, P0, P1, T0, T1};
use forest_hills_departure;
use heapless::{consts::*, Vec};
use stm32f3xx_hal::delay;

static TRAIN_TIMES: Resource<Cell<Option<Vec<chrono::NaiveTime, U10>>>, C1>;
static NOW: Resource<Cell<chrono::NaiveTime>, C1>;
static mut DELAY: delay::Delay;

// setup multitasking on microcontroller
tasks!(stm32f3xx_hal,
get_trains: Task{
    interrupt: Tim6,
    priority: P0,
    enabled: true
},
display_times: Task{
    interrupt: Time6,
    priority: P1,
    enabled: true
});

fn init(priority: P0, threshold: &TMax) {
    let now_plus_5 = chrono::NaiveTime::from_hms(12, 32, 30);
    let mut train_vec: Vec<chrono::NaiveTime, U10> = Vec::new();
    train_vec.push(now_plus_5);
    threshold.raise(&TRAIN_TIMES, |threshold: T1| {
        let train_times = TRAIN_TIMES.access(&priority, threshold);
        train_times.set(Some(train_vec));
    });
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f3xx_hal::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    unsafe {
        DELAY = delay::Delay::new(cp.SYST, clocks);
    }
}

fn idle(priority: P0, threshold: T0) {
    loop {
        cortex_m_rtfm::wfi()
    }
}

fn get_trains(priority: P0, threshold: T1) {
    loop {
        unsafe { DELAY.delay_ms(60_000u16) };
        // let new_train_times = forest_hills_departure::train_time::train_times()
        //     .unwrap_or_else(|err| panic!("{:?}", err));
        let now;
        threshold.raise(&NOW, |threshold: &T2| {
            now = NOW.access(&priority, threshold);
        });
        threshold.raise(&TRAIN_TIMES, |threshold: &T2| {
            let train_times = TRAIN_TIMES.access(&priority, threshold);
            if now > train_times.get()[0] {
                now_plus_5 = now + Duration::from_secs(300u64);
                train_times.set(now_plus_5);
            }
        });
        let mut old_train = train_times_clone.lock().unwrap();
        *old_train = new_train_times;
    }
}

fn display_times(priority: P1, threshold: T1) {
    let minimum_display_min = 5i64;
    // create a new clock struct, this initializes the display
    let mut clock = forest_hills_departure::ht16k33_clock::ClockDisplay::new();
    // create a new screen struct, this initializes the display
    let mut screen = forest_hills_departure::ssd1306_screen::ScreenDisplay::new();
    // clone the train_times to pass into thread
    loop {
        unsafe { DELAY.delay_ms(250u8) };
        // access and lock train times
        let train_times_unlocked = train_times_option.lock().unwrap();
        // if there are some train times, display on clock and screen
        let train_times_option = TRAIN_TIMES.access(&priority, &threshold);
        if let Some(train_times) = train_times_option {
            screen.display_trains(&train_times);
            clock.display_time_until(&train_times, &minimum_display_min);
        } else {
            // if there are no train times, clear both displays
            screen.clear_display(true);
            clock.clear_display();
        }
    }
}
