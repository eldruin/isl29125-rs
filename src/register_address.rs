use crate::{Error, Isl29125};
use embedded_hal::blocking::i2c;

pub const ADDR: u8 = 0b1000100;

pub struct Register;
impl Register {
    pub const DEVICE_ID: u8 = 0x00;
}

impl<I2C, E> Isl29125<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    pub(crate) fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data];
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
