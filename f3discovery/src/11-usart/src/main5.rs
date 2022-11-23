#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    loop {
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let byte = usart1.rdr.read().rdr().bits() as u8;

        // Echo the data
        usart1
            .tdr
            .write(|w| w.tdr().bits(u16::from(byte)));
        
        if byte == b'\r' {
            usart1
            .tdr
            .write(|w| w.tdr().bits(u16::from(b'\n')));
        }

        // aux11::bkpt();
    }
}