#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};
// Slave address
const MAGNETOMETER: u16 = 0b00111100;

// Addresses of the magnetometer's registers
#[allow(dead_code)]
const OUT_X_H_M: u8 = 0x03;
#[allow(dead_code)]
const IRA_REG_M: u8 = 0x0A;

// LSM303AGR
const WHO_AM_I_M: u8 = 0x4F;
const CFG_REG_A_M: u8 = 0x60;

fn set_continous(i2c1: &aux14::i2c1::RegisterBlock) {
    {
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });
        while i2c1.isr.read().txis().bit_is_clear() {}
        i2c1.txdr.write(|w| w.txdata().bits(CFG_REG_A_M));
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    let byte = {
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });
        while i2c1.isr.read().rxne().bit_is_clear() {}
        i2c1.rxdr.read().rxdata().bits()
    };

    let new_config = byte & 0b1111_1100;

    {
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(2);
            w.autoend().set_bit()
        });
        while i2c1.isr.read().txis().bit_is_clear() {}
        i2c1.txdr.write(|w| w.txdata().bits(CFG_REG_A_M));
        while i2c1.isr.read().txis().bit_is_clear() {}
        i2c1.txdr.write(|w| w.txdata().bits(new_config));
    }
}

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });

        // Wait until we can send more data
        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send the address of the register that we want to read: IRA_REG_M
        i2c1.txdr.write(|w| w.txdata().bits(WHO_AM_I_M));

        // Wait until the previous byte has been transmitted
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });

        // Wait until we have received the contents of the register
        while i2c1.isr.read().rxne().bit_is_clear() {}

        // Broadcast STOP (automatic because of `AUTOEND = 1`)

        i2c1.rxdr.read().rxdata().bits()
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    {
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });
        while i2c1.isr.read().txis().bit_is_clear() {}
        i2c1.txdr.write(|w| w.txdata().bits(CFG_REG_A_M));
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    let read_config = {
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });
        while i2c1.isr.read().rxne().bit_is_clear() {}
        i2c1.rxdr.read().rxdata().bits()
    };

    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", CFG_REG_A_M, read_config);

    set_continous(i2c1);

    {
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });
        while i2c1.isr.read().txis().bit_is_clear() {}
        i2c1.txdr.write(|w| w.txdata().bits(CFG_REG_A_M));
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    let read_config = {
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });
        while i2c1.isr.read().rxne().bit_is_clear() {}
        i2c1.rxdr.read().rxdata().bits()
    };

    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", CFG_REG_A_M, read_config);

    loop {}
}
