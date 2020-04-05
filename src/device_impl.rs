use crate::{Config, Error, Isl29125, Measurement, OperatingMode, Register};
use embedded_hal::blocking::i2c;

impl<I2C> Isl29125<I2C> {
    /// Create new instance of the device.
    pub fn new(i2c: I2C) -> Self {
        Isl29125 {
            i2c,
            config1: Config { bits: 0 },
        }
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
    /// Read all colors
    pub fn read(&mut self) -> Result<Measurement, Error<E>> {
        let mut data = [0; 6];
        self.read_data(Register::GREEN_L, &mut data)?;
        Ok(Measurement {
            green: u16::from(data[0]) | (u16::from(data[1]) << 8),
            red: u16::from(data[2]) | (u16::from(data[3]) << 8),
            blue: u16::from(data[4]) | (u16::from(data[5]) << 8),
        })
    }

    /// Set operating mode
    pub fn set_operating_mode(&mut self, mode: OperatingMode) -> Result<(), Error<E>> {
        let config1 = self.config1.bits & 0b1111_1000;
        let mask = match mode {
            OperatingMode::PowerDown => 0,
            OperatingMode::RedGreenBlue => 0b101,
        };
        let new_value = config1 | mask;
        self.write_register(Register::CONFIG1, new_value)?;
        self.config1.bits = new_value;
        Ok(())
    }

    /// Get device ID (`0x7D`)
    pub fn device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::DEVICE_ID)
    }

    /// Software reset
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::DEVICE_ID, 0x46)
    }
}
