#![no_std]

use byteorder::{BigEndian, ByteOrder};
use embedded_hal::{
    blocking::spi::{Transfer, Write},
    digital::OutputPin,
    spi::{Mode, MODE_1},
};

mod registers;
use registers::VolatileRegister;

pub const MODE: Mode = MODE_1;

pub struct As5047p<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, E> As5047p<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin,
{
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        Ok(As5047p { spi, cs })
    }

    pub fn get_mag(&mut self) -> Result<u16, E> {
        self.read_register(VolatileRegister::Mag)
    }

    pub fn get_angle(&mut self) -> Result<u16, E> {
        self.read_register(VolatileRegister::Angleunc)
    }

    pub fn get_angle_com(&mut self) -> Result<u16, E> {
        self.read_register(VolatileRegister::Anglecom)
    }

    pub fn read_register(&mut self, reg: VolatileRegister) -> Result<u16, E> {
        let mut buf = command_frame(reg.address(), Rw::Read);

        self.cs.set_low();
        self.spi.transfer(&mut buf)?;
        self.cs.set_high();

        Ok(read_frame(&mut buf))
    }
}

fn command_frame(addr: u16, rw: Rw) -> [u8; 2] {
    // [0:13] addr
    // [14] rw
    // [15] parity of [0:14]

    let mut frame = addr;

    // set/unset rw bit
    if rw.value() {
        frame |= 0b0100_0000_0000_0000;
    } else {
        frame &= 0b1011_1111_1111_1111;
    };

    // unset parity bit
    frame &= 0b0111_1111_1111_1111;
    if parity(frame) {
        frame |= 0b1000_0000_0000_0000;
    }

    let mut buf = [0; 2];
    BigEndian::write_u16(&mut buf, frame);
    buf
}

enum Rw {
    Read,
    Write,
}

impl Rw {
    fn value(&self) -> bool {
        match *self {
            Rw::Read => true,
            Rw::Write => false,
        }
    }
}

fn write_frame(data: u16) -> [u8; 2] {
    let mut frame = data;

    // unset parity and error
    frame &= 0b1111_1111_1111_1100;
    if parity(frame) {
        frame |= 0b0000_0000_0000_0001;
    }

    let mut buf = [0; 2];
    BigEndian::write_u16(&mut buf, frame);
    buf
}

fn read_frame(frame: &mut [u8]) -> u16 {
    // [0:13] data
    // [14] error
    // [15] parity of [0:14]
    if frame.len() != 2 {
        panic!("frame len greater than 2");
    }

    let temp = frame[0];
    frame[0] = frame[1];
    frame[1] = temp;

    let mut frame = BigEndian::read_u16(&frame);

    if parity(frame) {
        panic!("parity error when reading frame");
    }

    if frame & (1 << 14) != 0 {
        panic!("14th bit not zero in read frame");
    }

    frame &= 0b0011_1111_1111_1111;

    frame
}

fn parity(val: u16) -> bool {
    let mut val = val ^ (val >> 1);
    val = val ^ (val >> 2);
    val = val ^ (val >> 4);
    val = val ^ (val >> 8);

    if (val & 1) > 0 {
        return true;
    } else {
        return false;
    }
}
