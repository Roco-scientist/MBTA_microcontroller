extern crate ht16k33;
extern crate rppal;

fn main() {
    let i2c = rppal::i2c::I2c::new().unwrap();
    //    let address = 70u8;
    let address = 112u8; // actually 0x70 in hexidecimal which goes to 112
    let mut clock = ht16k33::HT16K33::new(i2c, address);
    let led_location = ht16k33::LedLocation::new(0, 0).unwrap();
    clock.set_led(led_location, true).unwrap();
}
