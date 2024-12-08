pub use avr_device::attiny2313 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_eeprom! {
    hal: crate::attiny2313,
    capacity: 128,
    addr_width: u8,
    addr_reg: eear,
}

impl_mod_port! {
    use crate::attiny2313 as hal;

    pub use avr_hal_generic::port::{mode, PinMode, PinOps};
    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: hal::pac::PORTA = [0, 1, 2],
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6],
        }
    }

    pub fn pins(peripherals: &hal::pac::Peripherals) -> Pins {
        return Pins::new(&peripherals.PORTA, &peripherals.PORTB, &peripherals.PORTD);
    }
}

impl_mod_wdt! {
    hal: crate::attiny2313,
    wdtcsr_name: wdtcr,
}

