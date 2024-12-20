#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests {
    use defmt_rtt as _;
    use esp_hal::{
        i2c::master::{AnyI2c, I2c},
        prelude::*,
        Blocking,
    };
    use tests_common::generic_setting_registers_tests::*;
    // Optional: A init function which is called before every test
    #[init]
    fn init() -> I2c<'static, Blocking, AnyI2c> {
        let peripherals = esp_hal::init({
            let mut config = esp_hal::Config::default();
            config.cpu_clock = CpuClock::max();
            config
        });
        let i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .with_sda(peripherals.GPIO5)
            .with_scl(peripherals.GPIO6);
        i2c
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
