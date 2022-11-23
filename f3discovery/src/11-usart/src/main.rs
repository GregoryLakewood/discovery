#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};
use heapless::Vec;

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\r\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\r\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // TODO implement this
        // hint: this will look very similar to the previous program
        for letter in s.bytes() {
            // wait until it's safe to write to TDR
            while self.usart1.isr.read().txe().bit_is_clear() {}

            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(letter)));
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    let mut serial = SerialPort { usart1 };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        // TODO Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.

        while serial.usart1.isr.read().rxne().bit_is_clear() {}

        let byte = serial.usart1.rdr.read().rdr().bits() as u8;

        match byte {
            b'\r' => {
                uprintln!(serial, "");
                for letter in buffer.iter().rev() {
                    // wait until it's safe to write to TDR
                    while serial.usart1.isr.read().txe().bit_is_clear() {}
                    serial.usart1.tdr.write(|w| w.tdr().bits(u16::from(*letter)));
                }
                uprintln!(serial, "");
                buffer.clear();
            },
            _ => {
                if buffer.push(byte).is_err() {
                    uprintln!(serial, "Buffer overflow. Reseting.");
                    buffer.clear();
                }
                // Echo the data
                while serial.usart1.isr.read().txe().bit_is_clear() {}
                serial.usart1.tdr.write(|w| w.tdr().bits(u16::from(byte)));
            },
        }

        // TODO Send back the reversed string
    }
}
