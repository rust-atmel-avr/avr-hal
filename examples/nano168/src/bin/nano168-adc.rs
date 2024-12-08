#![no_std]
#![no_main]

use arduino_hal::arduino::nano_v2 as board;
use board::prelude::*;
use panic_halt as _;

use board::adc;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let pins = board::pins!(dp);
    let mut serial = board::default_serial!(dp, pins, 57600);

    let mut adc = board::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        // The ATmega168 chip does not support the temperature functionality.
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).unwrap_infallible();

    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    let a2 = pins.a2.into_analog_input(&mut adc);
    let a3 = pins.a3.into_analog_input(&mut adc);
    let a4 = pins.a4.into_analog_input(&mut adc);
    let a5 = pins.a5.into_analog_input(&mut adc);

    loop {
        let values = [
            a0.analog_read(&mut adc),
            a1.analog_read(&mut adc),
            a2.analog_read(&mut adc),
            a3.analog_read(&mut adc),
            a4.analog_read(&mut adc),
            a5.analog_read(&mut adc),
        ];

        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).unwrap_infallible();
        }

        // Nano clone (with ATmega168) has two more ADC pins A6 and A7.  Accessing them works a bit different from
        // the other pins as they are not normal IO pins.  The code below shows how it works.
        let (a6, a7) = (
            adc.read_blocking(&adc::channel::ADC6),
            adc.read_blocking(&adc::channel::ADC7),
        );
        ufmt::uwrite!(&mut serial, "A6: {} A7: {}", a6, a7).unwrap_infallible();

        ufmt::uwriteln!(&mut serial, "").unwrap_infallible();
        board::delay_ms(1000);
    }
}
