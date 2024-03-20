# FlightCode

## System Architecture
### Timed Tasks
Timed Tasks are for anything that need to happen periodically in parallel. This is primarily used for polling data 
sources such as camera images and other sensors at a configurable rate.
\
Tasks can also multipurpose, such acting as an adapter for interfacing with different systems from while periodically running its control loop.
All while sending collecting data.

### Data Sources
TODO: list sources here

### Adapters
TODO: list device adapters here

### Services
Services are threaded logic loops that run in separate threads from the main one that serve specific purposes. 
They are initialized with a public spawn function that creates a thread that runs their main execution loop.

### Data Manager
The Data Manager is a service that runs in a separate thread from the main with the primary purpose of handling all incoming data 
and storing it locally into the system's filesystem. It also creates a globally accessible section of memory where it places the latest data of each source. 
Allowing any task or service to queue data for storage and to make globally available from anywhere within the application, while being completely thread safe.

### Payload Orientator
During cruise flight, images of the terrain below must be taken with the GoPro and thermal camera. There is a servo attached to the payload
that is responsible keeping the camera aimed straight onto the terrain below. FlightCode handles this by obtaining pitch and roll estimations 
from the PixHawk flight controller and adjusting the servo angles based off of these values.

### Battery Monitor
TODO: Might remove
The battery monitor ensures to warn the operator about an impending critical battery voltages, and will prevent flight 
while grounded and within configured voltage thresholds.

### REST API Service and Frontend


## APT Packages Dependencies
`sudo apt-get install -y libudev-dev pkg-config libssl-dev libopencv-dev clang libclang-dev`

## Deploy FlightCode and supporting services on On Board Computer (Raspberry Pi 5)
### Install systemd service and setup to run at startup
* `sudo cp system/services/* /etc/systemd/system/`
* `sudo systemctl daemon-reload`
* `sudo systemctl enable flightcode.service`
* `sudo systemctl enable ppp-serial-ethernet.service`
Now `flightcode.service` should run at boot and execute the release build of the FlightCode located at "/home/firedrone/FlightCode/target/release/flightcode". 

### Install udev rules for linking external devices to consistent device paths
* `sudo cp system/services/udev_rules/* /etc/udev/rules.d/`
* `sudo udevadm control --reload-rules`

### Services and FlightCode Application Execution
The systemd services locate and execute the release FlightCode executable by absolute path and executes other supporting programs. It is assumed that the user is "firedrone", if the user does not match this it will need to be updated in system/services/flightcode.service!

### Install Rust
* `sudo apt install curl -y`
* `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Clone and build release of FlightCode
In the "firedrone" user directory clone and build the release of FlightCode with the following commands.
* `git clone git@github.com:drone-capstone-gamers/FlightCode.git`
* `cd FlightCode`
* `cargo build`

### Build web frontend
Within the root of FlightCode, run the following commands.
* `cd src/application/rest_api_server/frontend`
* `npm install`
* `npm run build`
Now while FightCode is running, the frontend should be accessible in the browser via 'http://firedrone.local:8080' or whichever address the Raspberry Pi is given on the local network.

## Parameters
All FlightCode parameters are configurable inside of config.env. They are loaded automatically by the FlightCode systemd service on boot.
