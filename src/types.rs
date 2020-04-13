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
    /// Stand by (No ADC conversion)
    StandBy,
    /// Red only
    RedOnly,
    /// Green only
    GreenOnly,
    /// Blue only
    BlueOnly,
    /// Red/Green
    RedGreen,
    /// Green/Blue
    GreenBlue,
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

/// Interrupt pin (INT) mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptPinMode {
    /// Interrupts will be generated on exceeded thresholds. (ADC starts when writing to the config 1 register)
    Interrupt,
    /// INT pin is an input. ADC conversion starts on the rising edge at the INT pin.
    SyncStart,
}

impl Default for InterruptPinMode {
    fn default() -> Self {
        InterruptPinMode::Interrupt
    }
}

/// IR filtering range
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IRFilteringRange {
    /// Lower range (0-63, no offset)
    Lower,
    /// Higher range (0-63 corresponds to 106-169)
    Higher,
}

impl Default for IRFilteringRange {
    fn default() -> Self {
        IRFilteringRange::Lower
    }
}

/// Interrupt threshold assignment
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptThresholdAssignment {
    /// No interrupt (default)
    None,
    /// Use interrupt thresholds on red channel data.
    Red,
    /// Use interrupt thresholds on green channel data.
    Green,
    /// Use interrupt thresholds on blue channel data.
    Blue,
}

impl Default for InterruptThresholdAssignment {
    fn default() -> Self {
        InterruptThresholdAssignment::None
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

    #[test]
    fn can_get_default_int_pin_mode() {
        assert_eq!(InterruptPinMode::Interrupt, InterruptPinMode::default());
    }

    #[test]
    fn can_get_default_ir_filtering_range() {
        assert_eq!(IRFilteringRange::Lower, IRFilteringRange::default());
    }

    #[test]
    fn can_get_default_int_threshold_assignment() {
        assert_eq!(
            InterruptThresholdAssignment::None,
            InterruptThresholdAssignment::default()
        );
    }
}
