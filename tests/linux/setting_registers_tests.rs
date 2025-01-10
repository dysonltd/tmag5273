use crate::common::generic_setting_registers_tests::*;
use utils::setup_i2c;

#[test]
pub fn test_i2c_setup_success() {
    let i2c = setup_i2c();
    assert!(i2c.is_ok());
}

#[test]
fn test_is_connected() {
    let i2c = setup_i2c().unwrap();
    generic_test_is_connected(i2c);
}

#[test]
fn test_create_tmag5273() {
    let i2c = setup_i2c().unwrap();
    generic_test_create_tmag5273(i2c);
}

#[test]
fn test_set_reset_device_config_1_register() {
    let i2c = setup_i2c().unwrap();
    generic_test_set_reset_device_config_1_register(i2c);
}

#[test]
fn test_set_reset_device_config_2_register() {
    let i2c = setup_i2c().unwrap();
    generic_test_reset_device_config_2_register(i2c);
}

#[test]
fn test_set_reset_i2c_address_register() {
    let i2c = setup_i2c().unwrap();
    generic_test_set_reset_i2c_address_register(i2c);
}

#[test]
fn test_set_reset_int_config_1_register() {
    let i2c = setup_i2c().unwrap();
    generic_test_set_reset_int_config_1_register(i2c);
}

#[test]
fn test_set_reset_sensor_config_1_register() {
    let i2c = setup_i2c().unwrap();
    generic_test_set_reset_sensor_config_1_register(i2c);
}

#[test]
fn test_set_reset_sensor_config_2_register() {
    let i2c = setup_i2c().unwrap();
    generic_test_set_reset_sensor_config_2_register(i2c);
}

#[test]
fn test_set_reset_t_config_register() {
    let i2c = setup_i2c().unwrap();
    generic_test_set_reset_t_config_register(i2c);
}
