mod common;
use crate::common::{destroy, new, BitFlags as BF, Register, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use isl29125::{OperatingMode, Resolution};

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}

#[test]
fn can_get_device_id() {
    let dev_id = 0xAB;
    let mut sensor = new(&[I2cTrans::write_read(
        ADDR,
        vec![Register::DEVICE_ID],
        vec![0xAB],
    )]);
    let id = sensor.device_id().unwrap();
    assert_eq!(dev_id, id);
    destroy(sensor);
}

#[test]
fn can_reset() {
    let mut sensor = new(&[I2cTrans::write(ADDR, vec![Register::DEVICE_ID, 0x46])]);
    sensor.reset().unwrap();
    destroy(sensor);
}

#[test]
fn can_get_measurement() {
    let red = 0x1234;
    let green = 0x5678;
    let blue = 0x9ABC;
    let mut sensor = new(&[I2cTrans::write_read(
        ADDR,
        vec![Register::GREEN_L],
        vec![0x78, 0x56, 0x34, 0x12, 0xBC, 0x9A],
    )]);
    let m = sensor.read().unwrap();
    assert_eq!(m.red, red);
    assert_eq!(m.green, green);
    assert_eq!(m.blue, blue);
    destroy(sensor);
}

macro_rules! set_test {
    ($name:ident, $method:ident, $reg:ident, $value:expr $(, $arg:expr)*) => {
        #[test]
        fn $name() {
            let mut sensor = new(&[I2cTrans::write(ADDR, vec![Register::$reg, $value])]);
            sensor.$method($($arg),*).unwrap();
            destroy(sensor);
        }
    };
}

set_test!(
    set_mode_powerdown,
    set_operating_mode,
    CONFIG1,
    0,
    OperatingMode::PowerDown
);
set_test!(
    set_mode_rgb,
    set_operating_mode,
    CONFIG1,
    0b101,
    OperatingMode::RedGreenBlue
);

set_test!(
    set_resolution_12,
    set_resolution,
    CONFIG1,
    BF::RESOLUTION,
    Resolution::Bit12
);
set_test!(
    set_resolution_16,
    set_resolution,
    CONFIG1,
    0,
    Resolution::Bit16
);
