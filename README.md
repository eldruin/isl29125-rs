# Rust ISL29125 Digital RGB Color Light Sensor with IR Blocking Filter Driver

[![crates.io](https://img.shields.io/crates/v/isl29125.svg)](https://crates.io/crates/isl29125)
[![Docs](https://docs.rs/isl29125/badge.svg)](https://docs.rs/isl29125)
[![Build Status](https://travis-ci.com/eldruin/isl29125-rs.svg?branch=master)](https://travis-ci.com/eldruin/isl29125-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/isl29125-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/isl29125-rs?branch=master)

This is a platform agnostic Rust driver for the low-power digital RGB color
light sensor with IR blocking filter using the [`embedded-hal`] traits.

<!-- TODO
This driver allows you to:
-->
<!-- TODO
[Introductory blog post]()
-->

## The devices

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

<!-- TODO
## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
```
-->

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
