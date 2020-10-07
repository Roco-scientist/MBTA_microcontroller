#![no_std]

extern crate embedded_graphics;
extern crate rppal; // Crate for SPI, I2C, and GPIO on raspberry pi
extern crate sh1106; // Crate for current SPI oled display
extern crate std;

use embedded_graphics::prelude::*;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    style::TextStyleBuilder,
};
use rppal::{gpio, spi};
use sh1106::{mode::GraphicsMode, Builder};
use std::{thread, time};

fn main() {
    let spi0 = spi::Spi::new(
        spi::Bus::Spi0,
        spi::SlaveSelect::Ss0,
        400_000,
        spi::Mode::Mode0,
    )
    .unwrap();
    let spi_gpio = gpio::Gpio::new().unwrap();
    let spi_dc = spi_gpio.get(24).unwrap().into_output();
    let spi_cs = spi_gpio.get(8).unwrap().into_output();
    let mut screen_display: GraphicsMode<_> =
        Builder::new().connect_spi(spi0, spi_dc, spi_cs).into();
    screen_display.init().unwrap();
    screen_display.flush().unwrap();
    let text_style = TextStyleBuilder::new(Font6x8).build();
    screen_display.set_pixel(10, 10, 1u8);

    Text::new("Hello Rust!", Point::new(20, 30))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut screen_display)
        .unwrap();

    //    screen_display.draw(Font6x8::clone_into("Hello world!", 1u8.into()).into_iter());
    thread::sleep(time::Duration::from_secs(5));
    //    screen_display.draw(
    //        Font6x8::render_str("Hello Rust!")
    //            .translate(Coord::new(0, 16))
    //            .into_iter(),
    //    );
    Text::new("Hello World!", Point::new(20, 30))
        .into_styled(text_style)
        .draw(&mut screen_display)
        .unwrap();

    thread::sleep(time::Duration::from_secs(5));
    screen_display.flush().unwrap();
}
