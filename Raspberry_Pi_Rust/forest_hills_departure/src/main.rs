#![no_std]
extern crate chrono;

use forest_hills_departure;

fn main() {
    let train_times_option =
        forest_hills_departure::train_time::train_times().unwrap_or_else(|err| panic!("{:?}", err));
    let mut clock = forest_hills_departure::ClockDisplay::new();
    if let Some(train_times) = train_times_option {
        clock.display_time_until(train_times[0])
    } else {
        clock.clear_display()
    }
}
