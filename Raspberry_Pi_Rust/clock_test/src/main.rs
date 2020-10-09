extern crate rppal;
extern crate ht16k33;

fn main() {
    let i2c = rppal::i2c::I2c::new().unwrap();
    let address = 70u8;
    let mut clock = ht16k33::HT16K33::new(i2c, address);
    let led_location = ht16k33::LedLocation::new(0,0).unwrap();
    clock.set_led(led_location, true).unwrap();
}
