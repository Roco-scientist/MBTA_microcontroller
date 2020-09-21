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
<tr>GND<td></td><td>Ground</td><td>6</td><td>Ground</td></tr>
<tr>VCC<td></td><td>3.3 V</td><td>1</td><td>3.3 Volts</td></tr>
<tr>CLK<td></td><td>Clock</td><td>23</td><td>SCLK</td></tr>
<tr>MOSI<td></td><td>Data</td><td>19</td><td>SP10 MOSI</td></tr>
<tr>RES<td></td><td>Reset</td><td>22</td><td>GPIO25</td></tr>
<tr>DC<td></td><td></td><td>18</td><td>GPIO24</td></tr>
<tr>CS<td></td><td>CE0</td><td>24</td><td>CE0 (SPI)</td></tr>
</table>
## How to run
`python display.py`

