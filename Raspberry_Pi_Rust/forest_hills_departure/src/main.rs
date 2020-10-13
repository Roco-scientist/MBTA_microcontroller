#![no_std]

use forest_hills_departure;

fn main() {
    let train_times_option =
        forest_hills_departure::train_time::train_times().unwrap_or_else(|err| panic!("{:?}", err));
    let mut clock = forest_hills_departure::ClockDisplay::new();
    let mut screen = forest_hills_departure::ssd1306_screen::DisplayScreen::new(60u8);
    if let Some(train_times) = train_times_option {
        clock.display_time_until(&train_times[0]);
        screen.display_trains(&train_times)
    } else {
        clock.clear_display();
        screen.clear_display();
    }
}
