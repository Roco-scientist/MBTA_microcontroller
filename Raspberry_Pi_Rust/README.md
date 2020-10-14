# Rust version of MBTA controller for Raspberry Pi
## Requirements
<ul>
<li>Raspberry Pi (zero W). <a href=https://www.adafruit.com/product/3400>Can be purchased here</a></li>
<li>ssd1306 display with I2C connection (SPI would not connect with Rust crates). <a href=https://www.amazon.com/IZOKEE-Display-SSD1306-Raspberry-White-IIC/dp/B076PDVFQD/>Can be purchased here</a></li>
<li>7 segment clock display.  Adafruit 0.56" 4-Digit 7-Segment Display w/I2C Backpack. <a href=https://www.adafruit.com/product/3400>Can be purchased here</a></li>
<li> Misc. electronics for prototyping: bread board, wiring, multimeter etc.</li>
</ul>

## Connections

### Screen display
<table>
<tr><th>Label</th><th>Desc</th><th>RasPin</th><th>RasDesc</th></tr>
<tr><td>GND</td><td>Ground</td><td>6</td><td>Ground</td></tr>
<tr><td>VCC</td><td>3.3 V</td><td>1</td><td>3.3 Volts</td></tr>
<tr><td>SDA</td><td>SDA</td><td>38</td><td>GPIO20</td></tr>
<tr><td>SCL</td><td>SCL</td><td>40</td><td>GPIO21</td></tr>
</table>

The following line needs to be added to /boot/config.txt in order to enable another I2C bus<br>
`dtoverlay=i2c-gpio,bus=4,i2c_gpio_delay_us=1,i2c_gpio_sda=23,i2c_gpio_scl=24`

### Clock display
https://learn.adafruit.com/adafruit-led-backpack/0-dot-56-seven-segment-backpack-python-wiring-and-setup
<table>
<tr><th>Label</th><th>Desc</th><th>RasPin</th><th>RasDesc</th></tr>
<tr><td>GND</td><td>Ground</td><td>6</td><td>Ground</td></tr>
<tr><td>VCC</td><td>3.3 V</td><td>1</td><td>3.3 Volts</td></tr>
<tr><td>SDA</td><td>I2C SDA</td><td>3</td><td>GPIO2 (SDA)</td></tr>
<tr><td>SCL</td><td>I2C SCL</td><td>5</td><td>GPIO3 (SCL)</td></tr>
</table>

## How to run
`cargo build` or `cargo build --release`<br>
`./target/debug/forest_hills_departure`
### WARNING
Takes over 2 hours to compile in --release on Raspberry Pi 0
