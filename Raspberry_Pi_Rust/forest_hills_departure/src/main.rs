extern crate rppal;
extern crate std;

use forest_hills_departure;
use rppal::gpio;
use std::{
    sync::{Arc, Mutex},
    thread, time,
};

fn main() {
    let minimum_display_min = 5i64;
    // get the initial time trains and put them in a thread safe value to be passed back and forth
    // between threads
    let train_times_option = Arc::new(Mutex::new(
        forest_hills_departure::train_time::train_times()
            .unwrap_or_else(|err| eprintln!("ERROR - train_times - {}", err)),
    ));
    // create a new clock struct, this initializes the display
    let mut clock = forest_hills_departure::ht16k33_clock::ClockDisplay::new(0x70)
        .unwrap_or_else(|err| eprintln!("ERROR - ClockDisplay - {}", err));
    // create a new screen struct, this initializes the display
    let mut screen = forest_hills_departure::ssd1306_screen::ScreenDisplay::new(0x51)
        .unwrap_or_else(|err| eprintln!("ERROR - ScreenDisplay - {}", err));
    // clone the train_times to pass into thread
    let train_times_clone = Arc::clone(&train_times_option);
    // In a new thread find train times every minute and replace train_times with new value
    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(60));
        let new_train_times = forest_hills_departure::train_time::train_times()
            .unwrap_or_else(|err| eprintln!("ERROR - train_times - {}", err));
        let mut old_train = train_times_clone.lock().unwrap();
        *old_train = new_train_times;
    });
    // continually update screen and clock every 0.25 seconds
    loop {
        thread::sleep(time::Duration::from_millis(250));
        // access and lock train times
        let train_times_unlocked = train_times_option.lock().unwrap();
        // if there are some train times, display on clock and screen
        if let Some(train_times) = &*train_times_unlocked {
            screen
                .display_trains(&train_times)
                .unwrap_or_else(|err| eprintln!("ERROR - display_trains - {}", err));
            clock
                .display_time_until(&train_times, &minimum_display_min)
                .unwrap_or_else(|err| eprintln!("ERROR - display_time_until - {}", err));
        } else {
            // if there are no train times, clear both displays
            screen
                .clear_display(true)
                .unwrap_or_else(|err| eprintln!("ERROR - clear_display - {}", err));
            clock
                .clear_display()
                .unwrap_or_else(|err| eprintln!("ERROR - clear_display - {}", err));
        }
    }
}
