/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C communication error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// Measurement result
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// Red
    pub red: u16,
    /// Green
    pub green: u16,
    /// Blue
    pub blue: u16,
}

/// Operating mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatingMode {
    /// Power-Down (ADC conversion)
    PowerDown,
    /// Red/Green/Blue
    RedGreenBlue,
}

impl Default for OperatingMode {
    fn default() -> Self {
        OperatingMode::PowerDown
    }
}

/// ADC resolution
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolution {
    /// 12-bit resolution
    Bit12,
    /// 16-bit resolution (default)
    Bit16,
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::Bit16
    }
}

/// RGB data sensing range
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Range {
    /// 375 lux
    Lux375,
    /// 10000 lux
    Lux10000,
}

impl Default for Range {
    fn default() -> Self {
        Range::Lux375
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_default_operating_mode() {
        assert_eq!(OperatingMode::PowerDown, OperatingMode::default());
    }

    #[test]
    fn can_get_default_resolution() {
        assert_eq!(Resolution::Bit16, Resolution::default());
    }

    #[test]
    fn can_get_default_range() {
        assert_eq!(Range::Lux375, Range::default());
    }
}
