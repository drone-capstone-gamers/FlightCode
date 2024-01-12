# FlightCode

## Deps
`sudo apt-get install -y  libudev-dev`

## Deploy FlightCode on On Board Computer (Raspberry Pi 5)
### Install systemd service and setup to run at startup
* `sudo cp system/flightcode.service /etc/systemd/system/flightcode.service`
* `sudo systemctl daemon-reload`
* `sudo systemctl enable flightcode.service`
Now this service should run at boot and execute the release build of the FlightCode located at "/home/firedrone/FlightCode/target/release/flightcode".

### Clone and build release of FlightCode
The systemd service locates and executes the FlightCode executable by absolute path. It is assume that the user is "firedrone", if the user does not match this it will need to be updated in system/flightcode.service!
\
\
In the "firedrone" user directory clone and build the release of FlightCode with the following commands.
* `git clone git@github.com:drone-capstone-gamers/FlightCode.git`
* `cd FlightCode`
* `cargo build --release`
