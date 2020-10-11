#![no_std]
extern crate chrono;

use forest_hills_departure;

fn main() {
    let train_time = chrono::Utc::now().checked_add_signed(chrono::Duration::minutes(5));
    let mut clock = forest_hills_departure::ClockDisplay::new();
    clock.display_time_until(train_time)
}
