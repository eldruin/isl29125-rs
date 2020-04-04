//! This is a platform agnostic Rust driver for the low-power digital RGB color
//! light sensor with IR blocking filter using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! <!-- TODO
//! This driver allows you to:
//! -->
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The devices
//!
//! The ISL29125 is a low power, high sensitivity, RED, GREEN and BLUE color
//! light sensor (RGB) with an I2C (SMBus compatible) interface. Its
//! state-of-the-art photodiode array provides an accurate RGB spectral
//! response and excellent light source to light source variation (LS2LS).
//!
//! The ISL29125 is designed to reject IR in light sources allowing the device
//! to operate in environments from sunlight to dark rooms. The integrating
//! ADC rejects 50Hz and 60Hz flicker caused by artificial light sources.
//! A selectable range allows the user to optimize sensitivity suitable for
//! the specific application. In normal operation mode the device consumes
//! 56μA, which reduces to 0.5μA in power-down mode.
//!
//! The ISL29125 supports hardware and software user programmable interrupt
//! thresholds. The Interrupt persistency feature reduces false trigger
//! notification.
//!
//! Datasheet: [ISL29125](https://www.renesas.com/eu/en/www/doc/datasheet/isl29125.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! <!-- TODO -->
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]

mod device_impl;
mod types;
pub use crate::types::Error;
mod register_address;
use crate::register_address::Register;

/// ISL29125 device driver
#[derive(Debug)]
pub struct Isl29125<I2C> {
    i2c: I2C,
}

#[derive(Debug, Default, Clone, Copy)]
struct Config {
    bits: u8,
}
