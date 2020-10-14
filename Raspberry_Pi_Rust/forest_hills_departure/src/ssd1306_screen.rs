extern crate chrono;
extern crate embedded_graphics;
extern crate rppal; // Crate for SPI, I2C, and GPIO on raspberry pi
extern crate ssd1306; // Crate for current I2C oled display

use chrono::{DateTime, Local, prelude::*};
use embedded_graphics::{
    prelude::*,
    fonts::{Font12x16, Text},
    pixelcolor::BinaryColor,
    style::TextStyleBuilder,
};
use rppal::i2c;
use ssd1306::{prelude::*, Builder, I2CDIBuilder};

pub struct ScreenDisplay {
    display: GraphicsMode<I2CInterface<i2c::I2c>>,
    train1: Option<DateTime<Local>>,
    train2: Option<DateTime<Local>>,
}

impl ScreenDisplay {
    pub fn new() -> ScreenDisplay {
        let i2c4 = i2c::I2c::with_bus(4u8).unwrap();
        // let address = 60u8;
        let interface = I2CDIBuilder::new().init(i2c4);
        let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
        disp.init().unwrap();
        return ScreenDisplay{display: disp, train1: None, train2: None};
    }
    pub fn display_trains(&mut self, train_times: &Vec<DateTime<Local>>) -> () {
        let mut update_screen = false;
        if Some(train_times[0]) != self.train1 {
            self.train1 = Some(train_times[0].clone());
            update_screen = true;
        }
        if train_times.len() > 1 {
            if Some(train_times[1]) != self.train2 {
                self.train2 = Some(train_times[1].clone());
                update_screen = true;
            }
        } else {
            self.train2 = None
        }
        if update_screen {
            self.clear_display();
            let text_style = TextStyleBuilder::new(Font12x16)
                .text_color(BinaryColor::On)
                .build();
            if let Some(train1) = self.train1 {
                let hour = train1.hour();
                let minute = train1.minute();
                let time = format!("{}:{}", hour, minute);
                Text::new(&time, Point::new(35, 5))
                    .into_styled(text_style)
                    .draw(&mut self.display)
                    .unwrap();
                self.display.flush().unwrap();
            }
            if let Some(train2) = self.train2 {
                let hour = train2.hour();
                let minute = train2.minute();
                let time = format!("{}:{}", hour, minute);
                Text::new(&time, Point::new(35, 25))
                    .into_styled(text_style)
                    .draw(&mut self.display)
                    .unwrap();
                self.display.flush().unwrap();
            }
        }
    }
    pub fn clear_display(&mut self) -> () {
        self.display.clear();
        self.display.flush().unwrap();
    }
}
