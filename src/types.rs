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

#[cfg(test)]
mod tests {
    use super::OperatingMode;

    #[test]
    fn can_get_default_operating_mode() {
        assert_eq!(OperatingMode::PowerDown, OperatingMode::default());
    }
}
