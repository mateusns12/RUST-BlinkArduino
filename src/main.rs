#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripheral = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripheral);
    let mut led = pins.d13.into_output().downgrade();

    led.set_low();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
