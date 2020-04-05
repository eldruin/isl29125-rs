mod common;
use crate::common::{destroy, new, Register, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use isl29125::OperatingMode;

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

#[test]
fn can_set_mode_powerdown() {
    let mut sensor = new(&[I2cTrans::write(ADDR, vec![Register::CONFIG1, 0])]);
    sensor.set_operating_mode(OperatingMode::PowerDown).unwrap();
    destroy(sensor);
}

#[test]
fn can_set_mode_rgb() {
    let mut sensor = new(&[I2cTrans::write(ADDR, vec![Register::CONFIG1, 0b101])]);
    sensor
        .set_operating_mode(OperatingMode::RedGreenBlue)
        .unwrap();
    destroy(sensor);
}
