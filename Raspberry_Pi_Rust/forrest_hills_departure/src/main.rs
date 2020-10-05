#![no_std]

extern crate embedded_graphics;
extern crate rppal; // Crate for SPI, I2C, and GPIO on raspberry pi
extern crate sh1106; // Crate for current SPI oled display

use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
use rppal::spi;
use sh1106::{mode::GraphicsMode, Builder};
use std::{thread, time};


fn main() {
    let spi0 = spi::Spi::new(
        bus = spi::Bus::Spi0,
        slave_select = spi::SlaveSelect::Ss0,
        clock_speed = 400_000,
        mode = spi::Mode::Mode0,
    );
    let screen_display: GraphicsMode<_> = Builder::new().connect_spi(spi0).into();
    screen_display.init().unwrap();
    screen_display.flush().unwrap();
    screen_display.draw(Font6x8::render_str("Hello world!", 1u8.into()).into_iter());
    thread::sleep(time::Duration::from_seconds(5));
    screen_display.draw(
        Font6x8::render_str("Hello Rust!")
            .translate(Coord::new(0, 16))
            .into_iter(),
    );
    thread::sleep(time::Duration::from_seconds(5));
    screen_display.flush().unwrap();
}
