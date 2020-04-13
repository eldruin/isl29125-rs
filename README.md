# Rust ISL29125 Digital RGB Color Light Sensor with IR Blocking Filter Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/isl29125.svg)](https://crates.io/crates/isl29125)
[![Docs](https://docs.rs/isl29125/badge.svg)](https://docs.rs/isl29125)
-->
[![Build Status](https://travis-ci.com/eldruin/isl29125-rs.svg?branch=master)](https://travis-ci.com/eldruin/isl29125-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/isl29125-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/isl29125-rs?branch=master)

This is a platform agnostic Rust driver for the low-power digital RGB color
light sensor with IR blocking filter using the [`embedded-hal`] traits.

This driver allows you to:
- Read all colors. See: `read()`.
- Read red/green/blue colors individually. See: `red()`.
- Set operating mode. See: `set_operating_mode()`.
- Set ADC resolution. See: `set_resolution()`.
- Set RGB data sensing range. See: `set_range()`.
- Set IR filtering. See: `set_ir_filtering()`.
- Read the status flags. See: `status()`.
- Clear the status flags. See: `clear_status()`.
- Read the device ID. See: `device_id()`.
- Perform a software reset. See: `reset()`.
- Interrupts:
    - Set interrupt thresholds. See: `set_interrupt_thresholds()`.
    - Set interrupt threshold assignment. See: `set_interrupt_threshold_assignment()`.
    - Set the fault count. See: `set_fault_count()`.
    - Set interrupt pin mode. See: `set_interrupt_pin_mode()`.
    - Enable/Disable generating an interrupt after a conversion is done. See: `enable_interrupt_on_conversion_done()`.

<!-- TODO
[Introductory blog post]()
-->

## The device

The ISL29125 is a low power, high sensitivity, RED, GREEN and BLUE color
light sensor (RGB) with an I2C (SMBus compatible) interface. Its
state-of-the-art photodiode array provides an accurate RGB spectral
response and excellent light source to light source variation (LS2LS).

The ISL29125 is designed to reject IR in light sources allowing the device
to operate in environments from sunlight to dark rooms. The integrating
ADC rejects 50Hz and 60Hz flicker caused by artificial light sources.
A selectable range allows the user to optimize sensitivity suitable for
the specific application. In normal operation mode the device consumes
56μA, which reduces to 0.5μA in power-down mode.

The ISL29125 supports hardware and software user programmable interrupt
thresholds. The Interrupt persistency feature reduces false trigger
notification.

Datasheet: [ISL29125](https://www.renesas.com/eu/en/www/doc/datasheet/isl29125.pdf)


## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate linux_embedded_hal as hal;
use isl29125::{Isl29125, OperatingMode};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Isl29125::new(dev);
    sensor
        .set_operating_mode(OperatingMode::RedGreenBlue)
        .unwrap();
    loop {
        let m = sensor.read().unwrap();
        println!("R: {}, G: {}, B: {}", m.red, m.green, m.blue);
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/isl29125-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
