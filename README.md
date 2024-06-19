iryna1
======

Rust project for the _Arduino Uno_.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


## Setup notes

```
# https://blog.logrocket.com/complete-guide-running-rust-arduino/


# These commands used to setup rust+arduino developent

rustup toolchain install nightly
sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
cargo +stable install ravedude
cargo install cargo-generate

# These used to create a project
cargo generate --git https://github.com/Rahix/avr-hal-template.git

# To build
cargo build

# To find port, plug in usb and look at dmesg (look for something like ttyACM0, then confirm that device is in /dev/...)
sudo dmesg

# Open permissions
sudo chmod a+rw /dev/ttyACM0

# To flash...
export RAVEDUDE_PORT=/dev/ttyACM0
cargo run

```