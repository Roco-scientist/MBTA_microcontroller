# Purpose
The purpose of this repo is for code that will report to a mini clock display the next arriving train to forest hills station. <br>
<ul>
  <li>First prototype on the easiest system.  Python and raspberry pi pulling from MBTA API</li>
  <li>Then rewrite python code into Rust along with drivers</li>
  <li><strike>Repurpose Rust code to specialized ARM based microcontroller</strike></li>
</ul>

## Conclusion
Because of the reliance of this script on time, an internet connection, and a web API, 
it does not seem to be a good fit for bare metal embedding where local time does not exists and adding a web interface is complex.
For the microcontroller, I've included the Rust code up to the point where I gave up.  It is not fully functional though.<br>
<br>
I may come back to this with a DS3231 RTC module and an ESP8266 WiFi module.
