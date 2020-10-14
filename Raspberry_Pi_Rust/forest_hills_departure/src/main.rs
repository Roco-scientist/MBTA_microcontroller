#![no_std]

use forest_hills_departure;

fn main() {
    let train_times_option =
        forest_hills_departure::train_time::train_times().unwrap_or_else(|err| panic!("{:?}", err));
    let mut clock = forest_hills_departure::ht16k33_clock::ClockDisplay::new();
    let mut screen = forest_hills_departure::ssd1306_screen::ScreenDisplay::new();
    if let Some(train_times) = train_times_option {
        screen.display_trains(&train_times);
        clock.display_time_until(&train_times[0]);
    } else {
        screen.clear_display();
        clock.clear_display();
    }
    screen.clear_display();
    clock.clear_display();
}
