//! This is a platform agnostic Rust driver for the low-power digital RGB color
//! light sensor with IR blocking filter using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read all colors. See: [`read()`].
//! - Read red/green/blue colors individually. See: [`red()`].
//! - Set operating mode. See: [`set_operating_mode()`].
//! - Set ADC resolution. See: [`set_resolution()`].
//! - Set RGB data sensing range. See: [`set_range()`].
//! - Set IR filtering. See: [`set_ir_filtering()`].
//! - Read the status flags. See: [`status()`].
//! - Clear the status flags. See: [`clear_status()`].
//! - Read the device ID. See: [`device_id()`].
//! - Perform a software reset. See: [`reset()`].
//! - Interrupts:
//!     - Set interrupt thresholds. See: [`set_interrupt_thresholds()`].
//!     - Set interrupt threshold assignment. See: [`set_interrupt_threshold_assignment()`].
//!     - Set the fault count. See: [`set_fault_count()`].
//!     - Set interrupt pin mode. See: [`set_interrupt_pin_mode()`].
//!     - Enable/Disable generating an interrupt after a conversion is done. See: [`enable_interrupt_on_conversion_done()`].
//!
//! [`read()`]: struct.Isl29125.html#method.read
//! [`red()`]: struct.Isl29125.html#method.red
//! [`set_operating_mode()`]: struct.Isl29125.html#method.set_operating_mode
//! [`set_resolution()`]: struct.Isl29125.html#method.set_resolution
//! [`set_range()`]: struct.Isl29125.html#method.set_range
//! [`set_ir_filtering()`]: struct.Isl29125.html#method.set_ir_filtering
//! [`status()`]: struct.Isl29125.html#method.status
//! [`clear_status()`]: struct.Isl29125.html#method.clear_status
//! [`device_id()`]: struct.Isl29125.html#method.device_id
//! [`reset()`]: struct.Isl29125.html#method.reset
//! [`set_interrupt_thresholds()`]: struct.Isl29125.html#method.set_interrupt_thresholds
//! [`set_interrupt_threshold_assignment()`]: struct.Isl29125.html#method.set_interrupt_threshold_assignment
//! [`set_fault_count()`]: struct.Isl29125.html#method.set_fault_count
//! [`set_interrupt_pin_mode()`]: struct.Isl29125.html#method.set_interrupt_pin_mode
//! [`enable_interrupt_on_conversion_done()`]: struct.Isl29125.html#method.enable_interrupt_on_conversion_done
//!
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The device
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
//! ### Enable reading RGB and print the values
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use isl29125::{Isl29125, OperatingMode};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Isl29125::new(dev);
//! sensor
//!     .set_operating_mode(OperatingMode::RedGreenBlue)
//!     .unwrap();
//! loop {
//!     let m = sensor.read().unwrap();
//!     println!("R: {}, G: {}, B: {}", m.red, m.green, m.blue);
//! }
//! # }
//! ```
//!
//! ### Measure only the red color
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use isl29125::{Isl29125, OperatingMode};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Isl29125::new(dev);
//! sensor
//!     .set_operating_mode(OperatingMode::RedOnly)
//!     .unwrap();
//! loop {
//!     let red = sensor.red().unwrap();
//!     println!("Red: {}", red);
//! }
//! # }
//! ```
//!
//! ### Print the green color when the thresholds are exceeded 4 times
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use isl29125::{
//!     Isl29125, OperatingMode, FaultCount, InterruptThresholdAssignment
//! };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Isl29125::new(dev);
//! sensor
//!     .set_operating_mode(OperatingMode::GreenOnly)
//!     .unwrap();
//! let ita = InterruptThresholdAssignment::Green;
//! sensor.set_interrupt_threshold_assignment(ita).unwrap();
//! sensor.set_fault_count(FaultCount::Four).unwrap();
//! sensor.set_interrupt_thresholds(150, 8500).unwrap();
//! loop {
//!     while !sensor.status().unwrap().interrupt_triggered {}
//!     let green = sensor.green().unwrap();
//!     println!("Green: {}", green);
//! }
//! # }
//! ```
//!
//! ### Set the IR filtering
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use isl29125::{Isl29125, IRFilteringRange};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Isl29125::new(dev);
//! sensor
//!     .set_ir_filtering(IRFilteringRange::Lower(35))
//!     .unwrap();
//! # }
//! ```
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]

mod device_impl;
mod types;
pub use crate::types::{
    ConversionStatus, Error, FaultCount, IRFilteringRange, InterruptPinMode,
    InterruptThresholdAssignment, Measurement, OperatingMode, Range, Resolution, Status,
};
mod register_address;
use crate::register_address::{BitFlags, Register};

/// ISL29125 device driver
#[derive(Debug)]
pub struct Isl29125<I2C> {
    i2c: I2C,
    config1: Config,
    config3: Config,
}

#[derive(Debug, Default, Clone, Copy)]
struct Config {
    bits: u8,
}
