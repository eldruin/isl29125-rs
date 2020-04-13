use crate::{
    BitFlags, Config, Error, FaultCount, IRFilteringRange, InterruptPinMode,
    InterruptThresholdAssignment, Isl29125, Measurement, OperatingMode, Range, Register,
    Resolution,
};
use embedded_hal::blocking::i2c;

impl<I2C> Isl29125<I2C> {
    /// Create new instance of the device.
    pub fn new(i2c: I2C) -> Self {
        Isl29125 {
            i2c,
            config1: Config { bits: 0 },
            config3: Config { bits: 0 },
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
            OperatingMode::GreenOnly => 1,
            OperatingMode::RedOnly => 2,
            OperatingMode::BlueOnly => 3,
            OperatingMode::StandBy => 4,
            OperatingMode::RedGreenBlue => 5,
            OperatingMode::RedGreen => 6,
            OperatingMode::GreenBlue => 7,
        };
        self.set_config1(Config {
            bits: config1 | mask,
        })
    }

    /// Set ADC resolution
    pub fn set_resolution(&mut self, resolution: Resolution) -> Result<(), Error<E>> {
        let config1 = match resolution {
            Resolution::Bit12 => self.config1.with_high(BitFlags::RESOLUTION),
            Resolution::Bit16 => self.config1.with_low(BitFlags::RESOLUTION),
        };
        self.set_config1(config1)
    }

    /// Set RGB data sensing range
    pub fn set_range(&mut self, range: Range) -> Result<(), Error<E>> {
        let config1 = match range {
            Range::Lux375 => self.config1.with_low(BitFlags::RANGE),
            Range::Lux10000 => self.config1.with_high(BitFlags::RANGE),
        };
        self.set_config1(config1)
    }

    /// Set IR filtering
    ///
    /// The IR adjust must be a value in the range `[0-63]`. Providing a
    /// value outside this range will return `Error::InvalidInputData`.
    pub fn set_ir_filtering(
        &mut self,
        range: IRFilteringRange,
        ir_adjust: u8,
    ) -> Result<(), Error<E>> {
        if ir_adjust > 63 {
            Err(Error::InvalidInputData)
        } else {
            let ir_comp = match range {
                IRFilteringRange::Lower => ir_adjust,
                IRFilteringRange::Higher => BitFlags::IR_OFFSET | ir_adjust,
            };
            self.write_register(Register::CONFIG2, ir_comp)
        }
    }

    /// Set interrupt pin (INT) mode (Interrupt / Synced conversion start)
    pub fn set_interrupt_pin_mode(&mut self, mode: InterruptPinMode) -> Result<(), Error<E>> {
        let config1 = match mode {
            InterruptPinMode::Interrupt => self.config1.with_low(BitFlags::SYNC),
            InterruptPinMode::SyncStart => self.config1.with_high(BitFlags::SYNC),
        };
        self.set_config1(config1)
    }

    /// Set color channel used for threshold value interrupt generation
    pub fn set_interrupt_threshold_assignment(
        &mut self,
        assignment: InterruptThresholdAssignment,
    ) -> Result<(), Error<E>> {
        let config3 = self.config3.bits & 0b1111_1100;
        let config3 = match assignment {
            InterruptThresholdAssignment::None => config3,
            InterruptThresholdAssignment::Green => config3 | 1,
            InterruptThresholdAssignment::Red => config3 | 2,
            InterruptThresholdAssignment::Blue => config3 | 3,
        };
        self.set_config3(Config { bits: config3 })
    }

    /// Set number of consecutive fault events necessary to trigger an interrupt.
    /// This is referred to as "persistence" in the documentation.
    pub fn set_fault_count(&mut self, fault_count: FaultCount) -> Result<(), Error<E>> {
        let config3 = self.config3.bits & 0b1111_0011;
        let config3 = match fault_count {
            FaultCount::One => config3,
            FaultCount::Two => config3 | (1 << 2),
            FaultCount::Four => config3 | (2 << 2),
            FaultCount::Eight => config3 | (3 << 2),
        };
        self.set_config3(Config { bits: config3 })
    }

    fn set_config3(&mut self, config3: Config) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG3, config3.bits)?;
        self.config3 = config3;
        Ok(())
    }

    fn set_config1(&mut self, config1: Config) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG1, config1.bits)?;
        self.config1 = config1;
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

impl Config {
    fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}
