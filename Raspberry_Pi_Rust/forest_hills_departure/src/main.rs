extern crate std;

use forest_hills_departure;
use std::{sync::{Mutex, Arc}, thread, time};

fn main() {
    // get the initial time trains and put them in a thread safe value to be passed back and forth
    // between threads
    let train_times_option = Arc::new(Mutex::new(
        forest_hills_departure::train_time::train_times().unwrap_or_else(|err| panic!("{:?}", err)),
    ));
    // create a new clock struct, this initializes the display
    let mut clock = forest_hills_departure::ht16k33_clock::ClockDisplay::new();
    // create a new screen struct, this initializes the display
    let mut screen = forest_hills_departure::ssd1306_screen::ScreenDisplay::new();
    // clone the train_times to pass into thread
    let train_times_clone = Arc::clone(&train_times_option);
    // Find train times every minute and replace train_times with new value
    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(60));
        let new_train_times = forest_hills_departure::train_time::train_times()
            .unwrap_or_else(|err| panic!("{:?}", err));
        let mut old_train = train_times_clone.lock().unwrap();
        *old_train = new_train_times;
    });
    // continually update screen and clock every 0.25 seconds
    loop {
        thread::sleep(time::Duration::from_micros(250));
        // access and lock train times
        let train_times_unlocked = train_times_option.lock().unwrap();
        // if there are some train times, display on clock and screen
        if let Some(train_times) = &*train_times_unlocked {
            screen.display_trains(&train_times);
            clock.display_time_until(&train_times[0]);
        } else {
            // if there are no train times, clear both displays
            screen.clear_display();
            clock.clear_display();
        }
    }
}
