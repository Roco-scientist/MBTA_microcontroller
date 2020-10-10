#![no_std]
extern crate chrono;

use forest_hills_departure;

fn main() {
    let clock = forest_hills_departure::initialize_display().unwrap();
    let now = chrono::Utc::now();
    let train_time = now.checked_add_signed(chrono::Duration::minutes(5));
    forest_hills_departure::display_time(train_time, &mut clock);
}
