use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use isl29125::Isl29125;

pub struct Register;
#[allow(unused)]
impl Register {
    pub const DEVICE_ID: u8 = 0x00;
    pub const CONFIG1: u8 = 0x01;
    pub const GREEN_L: u8 = 0x09;
}
pub const ADDR: u8 = 0b100_0100;

#[allow(unused)]
pub fn new(transactions: &[I2cTrans]) -> Isl29125<I2cMock> {
    Isl29125::new(I2cMock::new(transactions))
}

#[allow(unused)]
pub fn destroy(sensor: Isl29125<I2cMock>) {
    sensor.destroy().done();
}
