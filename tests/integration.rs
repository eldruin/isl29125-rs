mod common;
use crate::common::{destroy, new, Register, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

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
