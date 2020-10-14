extern crate std;

use forest_hills_departure;
use std::{sync::Mutex, thread, time};

fn main() {
    let train_times_option = Mutex::new(
        forest_hills_departure::train_time::train_times().unwrap_or_else(|err| panic!("{:?}", err)),
    );
    let mut clock = forest_hills_departure::ht16k33_clock::ClockDisplay::new();
    let mut screen = forest_hills_departure::ssd1306_screen::ScreenDisplay::new();
    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(60));
        let new_train_times = forest_hills_departure::train_time::train_times()
            .unwrap_or_else(|err| panic!("{:?}", err));
        let mut old_train = train_times_option.lock().unwrap();
        *old_train = new_train_times;
    });
    loop {
        thread::sleep(time::Duration::from_micros(250));
        let train_times_unlocked = train_times_option.lock().unwrap();
        if let Some(train_times) = train_times_unlocked {
            screen.display_trains(&train_times);
            clock.display_time_until(&train_times[0]);
        } else {
            screen.clear_display();
            clock.clear_display();
        }
    }
    screen.clear_display();
    clock.clear_display();
}
