#![no_std]

extern crate embedded_graphics;
extern crate rppal; // Crate for SPI, I2C, and GPIO on raspberry pi
extern crate sh1106; // Crate for current SPI oled display
extern crate ssd1306;
extern crate std;

use embedded_graphics::prelude::*;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    style::{TextStyle, TextStyleBuilder},
};
use rppal::{gpio, spi};
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
    let mut screen_display_sh1106: sh1106::mode::graphics::GraphicsMode<_> = sh1106::Builder::new()
        .connect_spi(spi0, spi_dc, spi_cs)
        .into();
    screen_display_sh1106.init().unwrap();
    screen_display_sh1106.flush().unwrap();
    let text_style = TextStyleBuilder::new(Font6x8).build();
    screen_display_sh1106.set_pixel(10, 10, 1u8);

    Text::new("sh1106", Point::new(20, 30))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut screen_display_sh1106)
        .unwrap();

    //    screen_display.draw(Font6x8::clone_into("Hello world!", 1u8.into()).into_iter());
    thread::sleep(time::Duration::from_secs(1));
    screen_display_sh1106.flush().unwrap();

    let interface = ssd1306::prelude::SPIInterface::new(spi0, spi_dc, spi_cs);
    let mut screen_display_ssd1306: ssd1306::mode::graphics::GraphicsMode<_> =
        ssd1306::Builder::new().connect(interface).into();
    screen_display_ssd1306.set_pixel(15, 15, 1u8);
    screen_display_ssd1306.init().unwrap();
    screen_display_ssd1306.flush().unwrap();
    let text_style = TextStyleBuilder::new(Font6x8).build();
    screen_display_ssd1306.set_pixel(10, 10, 1u8);

    Text::new("ssd1306", Point::new(20, 30))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut screen_display_ssd1306)
        .unwrap();

    //    screen_display.draw(Font6x8::clone_into("Hello world!", 1u8.into()).into_iter());
    thread::sleep(time::Duration::from_secs(1));
    screen_display_ssd1306.flush().unwrap();
}
