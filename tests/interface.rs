mod common;
use crate::common::{BitFlags, DEFAULT_CONFIG1, DEV_ADDR, I2cTrans, Register, destroy, new};

#[test]
fn can_create() {
    let sensor = new(&[]);
    destroy(sensor);
}

#[test]
fn can_create_async() {
    let sensor = new(&[]);
    destroy(sensor);

    let mut mock = ::embedded_hal_mock::eh1::i2c::Mock::new(&[]);
    let sensor = ::apds9960::Apds9960::new_async(
        ::embassy_embedded_hal::adapter::BlockingAsync::new(&mut mock),
    );
    drop(sensor.destroy());
    mock.done();
}

write_test!(can_enable, enable, ENABLE, BitFlags::PON);
write_test!(can_disable, disable, ENABLE, 0);

read_test!(can_read_id, read_device_id, 0xAB, ID, 0xAB);

write_test!(can_enable_wait, enable_wait, ENABLE, BitFlags::WEN);
write_test!(can_disable_wait, disable_wait, ENABLE, 0);
write_test!(
    en_wlong,
    enable_wait_long,
    CONFIG1,
    DEFAULT_CONFIG1 | BitFlags::WLONG
);
write_test!(dis_wlong, disable_wait_long, CONFIG1, DEFAULT_CONFIG1);
write_test!(set_wtime, set_wait_time, WTIME, 0x0F, 0x0F);

empty_write_test!(force_int, force_interrupt, IFORCE);
empty_write_test!(clear_ints, clear_interrupts, AICLEAR);
