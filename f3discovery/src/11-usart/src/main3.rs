#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1::RegisterBlock};

const TEXT: &str = "The quick brown fox jumps over the lazy dog.";

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // Send a string
    for letter in TEXT.encode_utf16() {
        usart1.tdr.write(|w| w.tdr().bits(letter));
    }

    // Send a single character
    usart1
        .tdr
        .write(|w| w.tdr().bits(u16::from(b'X')) );

    loop {}
}
