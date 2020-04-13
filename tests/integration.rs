mod common;
use crate::common::{destroy, new, BitFlags as BF, Register, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use isl29125::{
    FaultCount, IRFilteringRange, InterruptPinMode, InterruptThresholdAssignment, OperatingMode,
    Range, Resolution,
};

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

macro_rules! set_operating_mode_test {
    ($name:ident, $value:expr, $mode:ident) => {
        set_test!(
            $name,
            set_operating_mode,
            CONFIG1,
            $value,
            OperatingMode::$mode
        );
    };
}
set_operating_mode_test!(set_mode_powerdown, 0, PowerDown);
set_operating_mode_test!(set_mode_green_only, 1, GreenOnly);
set_operating_mode_test!(set_mode_red_only, 2, RedOnly);
set_operating_mode_test!(set_mode_blue_only, 3, BlueOnly);
set_operating_mode_test!(set_mode_stand_by, 4, StandBy);
set_operating_mode_test!(set_mode_rgb, 5, RedGreenBlue);
set_operating_mode_test!(set_mode_red_green, 6, RedGreen);
set_operating_mode_test!(set_mode_green_blue, 7, GreenBlue);

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

set_test!(set_range_375, set_range, CONFIG1, 0, Range::Lux375);
set_test!(
    set_range_10000,
    set_range,
    CONFIG1,
    BF::RANGE,
    Range::Lux10000
);

set_test!(
    set_int_mode_int,
    set_interrupt_pin_mode,
    CONFIG1,
    0,
    InterruptPinMode::Interrupt
);
set_test!(
    set_int_mode_sync,
    set_interrupt_pin_mode,
    CONFIG1,
    BF::SYNC,
    InterruptPinMode::SyncStart
);

#[test]
fn cannot_set_wrong_ir_adjust() {
    let mut sensor = new(&[]);
    sensor
        .set_ir_filtering(IRFilteringRange::Lower, 64)
        .expect_err("Should return error.");
    destroy(sensor);
}

set_test!(
    set_ir_filtering_lower,
    set_ir_filtering,
    CONFIG2,
    63,
    IRFilteringRange::Lower,
    63
);

set_test!(
    set_ir_filtering_higher,
    set_ir_filtering,
    CONFIG2,
    BF::IR_OFFSET | 63,
    IRFilteringRange::Higher,
    63
);

macro_rules! set_int_th_assign_test {
    ($name:ident, $value:expr, $mode:ident) => {
        set_test!(
            $name,
            set_interrupt_threshold_assignment,
            CONFIG3,
            $value,
            InterruptThresholdAssignment::$mode
        );
    };
}

set_int_th_assign_test!(set_int_th_assign_none, 0, None);
set_int_th_assign_test!(set_int_th_assign_green, 1, Green);
set_int_th_assign_test!(set_int_th_assign_red, 2, Red);
set_int_th_assign_test!(set_int_th_assign_blue, 3, Blue);

macro_rules! set_fc_test {
    ($name:ident, $value:expr, $mode:ident) => {
        set_test!($name, set_fault_count, CONFIG3, $value, FaultCount::$mode);
    };
}
set_fc_test!(set_fault_count_one, 0, One);
set_fc_test!(set_fault_count_two, 1 << 2, Two);
set_fc_test!(set_fault_count_four, 2 << 2, Four);
set_fc_test!(set_fault_count_eight, 3 << 2, Eight);

set_test!(
    enable_int_on_conv_done,
    enable_interrupt_on_conversion_done,
    CONFIG3,
    BF::CONVEN
);
set_test!(
    disable_int_on_conv_done,
    disable_interrupt_on_conversion_done,
    CONFIG3,
    0
);
