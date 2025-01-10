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
    use tests_common::generic_setting_registers_tests::*;
    #[init]
    fn init() -> I2c<'static, Blocking, AnyI2c> {
        initialise()
    }

    #[test]
    fn test_is_connected(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_is_connected(i2c); // Pass the i2c variable to the inner test function
    }
    #[test]
    fn test_create_tmag5273(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_create_tmag5273(i2c); // Pass the i2c variable to the inner test function
    }
    #[test]
    fn test_set_reset_device_config_1_register(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_set_reset_device_config_1_register(i2c); // Pass the i2c variable to the inner test function
    }
    #[test]
    fn test_set_reset_device_config_2_register(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_reset_device_config_2_register(i2c); // Pass the i2c variable to the inner test function
    }
    #[test]
    fn test_set_reset_i2c_address_register(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_set_reset_i2c_address_register(i2c); // Pass the i2c variable to the inner test function
    }
    #[test]
    fn test_set_reset_int_config_1_register(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_set_reset_int_config_1_register(i2c); // Pass the i2c variable to the inner test function
    }
    #[test]
    fn test_set_reset_sensor_config_1_register(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_set_reset_sensor_config_1_register(i2c); // Pass the i2c variable to the inner test function
    }
    #[test]
    fn test_set_reset_sensor_config_2_register(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_set_reset_sensor_config_2_register(i2c); // Pass the i2c variable to the inner test function
    }
    #[test]
    fn test_set_reset_t_config_register(i2c: I2c<'static, Blocking, AnyI2c>) {
        generic_test_set_reset_t_config_register(i2c); // Pass the i2c variable to the inner test function
    }
}
