# Rust version of MBTA controller for Raspberry Pi
## Requirements
### Standard library for cross compiling
Raspberry pi 0/1:</br>
`rustup target add arm-unknown-linux-gnueabihf`
<br>
<br>
Raspberry pi 2/3/4: <br>
`rustup target add armv7-unknown-linux-gnueabihf`
<br>
### Tool chain to compile on a computer instead of on the raspberry pi
<a href=https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads>GNU Toolchain download location</a>
<ul>
<li>Pi 0/1: AArch32 target with hard float (arm-linux-gnueabihf)</li>
<li>Pi 2/3/4: AArch32 target with hard float (arm-none-linux-gnueabihf)</li>
</ul>
Add to PATH<br>
`export PATH="$HOME/PATH_TO_YOUR_DOWNLOAD/PATH_TO_TOOLCHAIN_FOLDER/bin:$PATH"`
<br>
## Run remotely
Create a bash script with below or export environmental variables
```
PI_IP=<raspberry_pi_IP> # update this 
TARGET=arm-unknown-linux-gnueabihf # Pi 0/1
# TARGET=armv7-unknown-linux-gnueabihf # Pi 2/3/4

# build binary
cargo build --target $TARGET

# upload binary
sshpass -p 'raspberry' scp -r ./target/$TARGET/debug/forrest_hills_departure pi@$PI_IP:/home/pi

# execute binary
sshpass -p 'raspberry' ssh pi@$PI_IP './forrest_hills_departure'
```
NOTE: Will not work with my current setup.  SSH login has been changed from pi
