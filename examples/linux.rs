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
