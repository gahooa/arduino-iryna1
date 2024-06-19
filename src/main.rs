#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut blue_pin = pins.d13.into_output();
    let mut green_pin = pins.d12.into_output();
    let mut red_pin = pins.d11.into_output();

    loop {
        // blue
        blue_pin.toggle();
        arduino_hal::delay_ms(1000);
        blue_pin.toggle();

        // green
        green_pin.toggle();
        arduino_hal::delay_ms(1000);
        green_pin.toggle();

        // red
        red_pin.toggle();
        arduino_hal::delay_ms(1000);
        red_pin.toggle();

        // blue + green
        blue_pin.toggle();
        green_pin.toggle();
        arduino_hal::delay_ms(1000);
        blue_pin.toggle();
        green_pin.toggle();

        // blue + red
        blue_pin.toggle();
        red_pin.toggle();
        arduino_hal::delay_ms(1000);
        blue_pin.toggle();
        red_pin.toggle();

        // green + red
        green_pin.toggle();
        red_pin.toggle();
        arduino_hal::delay_ms(1000);
        green_pin.toggle();
        red_pin.toggle();

        // blue + green + red
        blue_pin.toggle();
        green_pin.toggle();
        red_pin.toggle();
        arduino_hal::delay_ms(1000);
        blue_pin.toggle();
        green_pin.toggle();
        red_pin.toggle();
        

    }
}
