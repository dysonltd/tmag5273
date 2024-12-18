use crate::common::generic_cold_start_tests::*;
use utils::setup_i2c;
#[test]
pub fn test_device_id() {
    let i2c = setup_i2c().unwrap();
    generic_test_device_id(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_manufacturer_id() {
    let i2c = setup_i2c().unwrap();
    generic_test_manufacturer_id(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_registers() {
    let i2c = setup_i2c().unwrap();
    generic_test_registers(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_default_i2c_address() {
    let i2c = setup_i2c().unwrap();
    generic_test_default_i2c_address(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_get_magnitude_first_boot() {
    let i2c = setup_i2c().unwrap();
    generic_test_get_magnitude_first_boot(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_get_xyz_thresholds_first_boot() {
    let i2c = setup_i2c().unwrap();
    generic_test_get_xyz_thresholds_first_boot(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_magnetic_gain() {
    let i2c = setup_i2c().unwrap();
    generic_test_magnetic_gain(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_magnetic_offset_invalid_at_boot() {
    let i2c = setup_i2c().unwrap();
    generic_test_magnetic_offset_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_temperature_invalid_at_boot() {
    let i2c = setup_i2c().unwrap();
    generic_test_temperature_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_get_data_methods() {
    let i2c = setup_i2c().unwrap();
    generic_test_get_data_methods(i2c); // Pass the i2c variable to the inner test function
}

#[test]
fn test_get_angle() {
    let i2c = setup_i2c().unwrap();
    generic_test_get_angle(i2c); // Pass the i2c variable to the inner test function
}
