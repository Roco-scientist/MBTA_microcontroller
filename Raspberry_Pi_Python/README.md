# Python and Raspberry Pi
## Required hardware
Work in progress <br>
Required:<br>
<ul><li>Raspberry Pi (zero W)</li><li>sh1106 display with SPI (I2C will work with modification)</li></ul>

## Connections
Connections for SPI display to raspberry pi pins<br>
Will be different for I2C or parallel
<table>
<tr><th>Label</th><th>Desc</th><th>RasPin</th><th>RasDesc</th></tr>
<tr><td>GND</td><td>Ground</td><td>6</td><td>Ground</td></tr>
<tr><td>VCC</td><td>3.3 V</td><td>1</td><td>3.3 Volts</td></tr>
<tr><td>CLK</td><td>Clock</td><td>23</td><td>SCLK</td></tr>
<tr><td>MOSI</td><td>Data</td><td>19</td><td>SP10 MOSI</td></tr>
<tr><td>RES</td><td>Reset</td><td>22</td><td>GPIO25</td></tr>
<tr><td>DC</td><td></td><td>18</td><td>GPIO24</td></tr>
<tr><td>CS</td><td>CE0</td><td>24</td><td>CE0 (SPI)</td></tr>
</table>

## How to run
`python display.py`

