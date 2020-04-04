use crate::{Error, Isl29125, Register};
use embedded_hal::blocking::i2c;

impl<I2C> Isl29125<I2C> {
    /// Create new instance of the device.
    pub fn new(i2c: I2C) -> Self {
        Isl29125 { i2c }
    }

    /// Destroy driver instance, return I2C bus.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, E> Isl29125<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Get device ID (`0x7D`)
    pub fn device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::DEVICE_ID)
    }

    /// Software reset
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::DEVICE_ID, 0x46)
    }
}
