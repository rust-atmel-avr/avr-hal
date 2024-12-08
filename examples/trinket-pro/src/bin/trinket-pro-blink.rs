#![no_std]
#![no_main]

use arduino_hal::adafruit::trinket_pro as board;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let pins = board::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        led.toggle();
        board::delay_ms(100);
        led.toggle();
        board::delay_ms(100);
        led.toggle();
        board::delay_ms(100);
        led.toggle();
        board::delay_ms(800);
    }
}
