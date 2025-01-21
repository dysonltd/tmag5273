#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests {

    use esp32c3_tests::initialise::{initialise, EspI2c};
    use tests_common::generic_cold_start_tests::*;
    #[init]
    fn init() -> EspI2c {
        initialise()
    }

    #[test]
    fn test_device_id(i2c: EspI2c) {
        generic_test_device_id(i2c);
    }

    #[test]
    fn test_manufacturer_id(i2c: EspI2c) {
        generic_test_manufacturer_id(i2c);
    }

    #[test]
    fn test_registers(i2c: EspI2c) {
        generic_test_registers(i2c);
    }

    #[test]
    fn test_default_i2c_address(i2c: EspI2c) {
        generic_test_default_i2c_address(i2c);
    }

    #[test]
    fn test_get_magnitude_first_boot(i2c: EspI2c) {
        generic_test_get_magnitude_first_boot(i2c);
    }

    #[test]
    fn test_get_xyz_thresholds_first_boot(i2c: EspI2c) {
        generic_test_get_xyz_thresholds_first_boot(i2c);
    }

    #[test]
    fn test_magnetic_gain(i2c: EspI2c) {
        generic_test_magnetic_gain(i2c);
    }

    #[test]
    fn test_magnetic_offset_invalid_at_boot(i2c: EspI2c) {
        generic_test_magnetic_offset_invalid_at_boot(i2c);
    }

    #[test]
    fn test_temperature_invalid_at_boot(i2c: EspI2c) {
        generic_test_temperature_invalid_at_boot(i2c);
    }

    #[test]
    fn test_get_data_methods(i2c: EspI2c) {
        generic_test_get_data_methods(i2c);
    }

    #[test]
    fn test_get_angle(i2c: EspI2c) {
        generic_test_get_angle(i2c);
    }
}
