use crate::{Error, Isl29125};
use embedded_hal::blocking::i2c;

pub const ADDR: u8 = 0b100_0100;

pub struct Register;
impl Register {
    pub const DEVICE_ID: u8 = 0x00;
    pub const CONFIG1: u8 = 0x01;
    pub const CONFIG2: u8 = 0x02;
    pub const CONFIG3: u8 = 0x03;
    pub const THL: u8 = 0x04;
    pub const STATUS: u8 = 0x08;
    pub const GREEN_L: u8 = 0x09;
}

pub struct BitFlags;
impl BitFlags {
    pub const SYNC: u8 = 1 << 5;
    pub const RESOLUTION: u8 = 1 << 4;
    pub const RANGE: u8 = 1 << 3;
    pub const IR_OFFSET: u8 = 1 << 7;
    pub const CONVEN: u8 = 1 << 4;
    pub const BOUTF: u8 = 1 << 2;
    pub const CONVENF: u8 = 1 << 1;
    pub const RGBTHF: u8 = 1;
}

impl<I2C, E> Isl29125<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    pub(crate) fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data];
        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }

    pub(crate) fn write_thresholds(&mut self, low: u16, high: u16) -> Result<(), Error<E>> {
        let payload: [u8; 5] = [
            Register::THL,
            (low & 0xFF) as u8,
            ((low & 0xFF00) >> 8) as u8,
            (high & 0xFF) as u8,
            ((high & 0xFF00) >> 8) as u8,
        ];
        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }
}

impl<I2C, E> Isl29125<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    pub(crate) fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.read_data(register, &mut data).and(Ok(data[0]))
    }

    pub(crate) fn read_data(&mut self, register: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c
            .write_read(ADDR, &[register], data)
            .map_err(Error::I2C)
    }
}
