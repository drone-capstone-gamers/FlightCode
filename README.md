# FlightCode

## APT Packages Dependencies
`sudo apt-get install -y libudev-dev`

## Deploy FlightCode and supporting services on On Board Computer (Raspberry Pi 5)
### Install systemd service and setup to run at startup
* `sudo cp system/services/* /etc/systemd/system/`
* `sudo systemctl daemon-reload`
* `sudo systemctl enable flightcode.service`
* `sudo systemctl enable ppp-serial-ethernet.service`
Now `flightcode.service` should run at boot and execute the release build of the FlightCode located at "/home/firedrone/FlightCode/target/release/flightcode". 

### Services and FlightCode Application Execution
The systemd services locate and execute the release FlightCode executable by absolute path and executes other supporting programs. It is assumed that the user is "firedrone", if the user does not match this it will need to be updated in system/services/flightcode.service!

### Install Rust
sudo apt install curl -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### Clone and build release of FlightCode
In the "firedrone" user directory clone and build the release of FlightCode with the following commands.
* `git clone git@github.com:drone-capstone-gamers/FlightCode.git`
* `cd FlightCode`
* `cargo build --release`
