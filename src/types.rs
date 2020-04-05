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

/// Status
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Status {
    /// Whether data is ready
    pub data_ready: bool,
    /// Whether the temperature high threshold was exceeded
    pub high_temp_threshold_exceeded: bool,
    /// Whether the temperature low threshold was exceeded
    pub low_temp_threshold_exceeded: bool,
    /// Whether the humidity high threshold was exceeded
    pub high_humidity_threshold_exceeded: bool,
    /// Whether the humidity low threshold was exceeded
    pub low_humidity_threshold_exceeded: bool,
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
