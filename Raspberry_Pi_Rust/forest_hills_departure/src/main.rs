#![no_std]

use forest_hills_departure;

fn main() {
    let clock = forest_hills_departure::initialize_display().unwrap();
    forest_hills_departure::display_time("11:06", &mut clock);
}
