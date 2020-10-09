#![no_std]

extern crate embedded_graphics;
extern crate rppal; // Crate for SPI, I2C, and GPIO on raspberry pi
extern crate sh1106; // Crate for current SPI oled display
extern crate std;

use embedded_graphics::prelude::*;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    image::{Image, ImageRawLE},
    pixelcolor::BinaryColor,
    style::{TextStyle, TextStyleBuilder},
};
use rppal::{gpio, spi};
use sh1106::{prelude::*, Builder};
use std::{thread, time};

fn main() {
    let spi_gpio = gpio::Gpio::new().unwrap();
    let spi0 = spi::Spi::new(
        spi::Bus::Spi0,
        spi::SlaveSelect::Ss0,
        400_000,
        spi::Mode::Mode0,
    )
    .unwrap();
    let spi_dc = spi_gpio.get(24).unwrap().into_output();
    let spi_cs = spi_gpio.get(8).unwrap().into_output();
    let mut screen_display_sh1106: GraphicsMode<_> =
        Builder::new().connect_spi(spi0, spi_dc, spi_cs).into();
    screen_display_sh1106.init().unwrap();
    screen_display_sh1106.flush().unwrap();
    let im: ImageRawLE<BinaryColor> = ImageRawLE::new(include_bytes!("../rust.raw"), 64, 64);
    Image::new(&im, Point::new(32, 0))
        .draw(&mut screen_display_sh1106)
        .unwrap();
    screen_display_sh1106.flush().unwrap();
    thread::sleep(time::Duration::from_secs(1));
    screen_display_sh1106.set_pixel(10, 10, 1u8);
    screen_display_sh1106.flush().unwrap();
    thread::sleep(time::Duration::from_secs(1));
    Text::new("sh1106", Point::new(20, 30))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut screen_display_sh1106)
        .unwrap();
    screen_display_sh1106.flush().unwrap();
    thread::sleep(time::Duration::from_secs(1));
//    screen_display_sh1106.draw(Font6x8::clone_into("Hello world!", 1u8.into()).into_iter());
//    screen_display_sh1106.flush().unwrap();
//    thread::sleep(time::Duration::from_secs(1));
}
