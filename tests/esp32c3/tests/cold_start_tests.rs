#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests {

    use esp32c3_tests::initialise::initialise;
    use esp_hal::{
        i2c::master::{AnyI2c, I2c},
        Blocking,
    };
    use tests_common::generic_cold_start_tests::*;
    #[init]
    fn init() -> I2c<'static, Blocking, AnyI2c> {
        initialise()
    }

    #[test]
    fn test_device_id(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_device_id(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_manufacturer_id(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_manufacturer_id(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_registers(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_registers(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_default_i2c_address(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_default_i2c_address(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_magnitude_first_boot(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_get_magnitude_first_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_xyz_thresholds_first_boot(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_get_xyz_thresholds_first_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_magnetic_gain(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_magnetic_gain(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_magnetic_offset_invalid_at_boot(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_magnetic_offset_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_temperature_invalid_at_boot(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_temperature_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_data_methods(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_get_data_methods(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_angle(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_get_angle(i2c); // Pass the i2c variable to the inner test function
    }
}
