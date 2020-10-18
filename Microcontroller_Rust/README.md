# Microcontroller MBTA scheduler written in rust
## Required
<ul>
<li>STM micontroller.  So far written for ST32F3Discovery board</li>
<li>ssd1306 display with I2C connection (SPI would not connect with Rust crates). <a href=https://www.amazon.com/IZOKEE-Display-SSD1306-Raspberry-White-IIC/dp/B076PDVFQD/>Can be purchased here</a></li>
<li>7 segment clock display.  Adafruit 0.56" 4-Digit 7-Segment Display w/I2C Backpack. <a href=https://www.adafruit.com/product/3400>Can be purchased here</a></li>
</ul>

## WARNING
### Crate not fully working
### Making connection to the I2C interfaces still needs to be worked out
A microntroller likely will not work well for this project because
<ul>
<li>Microcontrollers don't contain a clock for time, therefor there are no crates for getting the current time to compare to train time</li>
<li>ESP8266 WiFi board does not yet communicate in Rust</li>
<li>Pulling a web API is complicated to interface</li>
<li>Maybe blue tooth with a separate server will fix some of this, TBD</li>
</ul>
