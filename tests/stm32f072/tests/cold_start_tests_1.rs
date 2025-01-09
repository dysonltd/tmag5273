#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests_1 {
    use defmt_rtt as _;
    use stm32f072::initialise::*;
    use tests_common::generic_cold_start_tests::*;

    #[init]
    fn init() -> Stm32I2c {
        initialise()
    }

    #[test]
    fn test_get_xyz_thresholds_first_boot(i2c: Stm32I2c) {
        generic_test_get_xyz_thresholds_first_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_magnetic_gain(i2c: Stm32I2c) {
        generic_test_magnetic_gain(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_magnetic_offset_invalid_at_boot(i2c: Stm32I2c) {
        generic_test_magnetic_offset_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_temperature_invalid_at_boot(i2c: Stm32I2c) {
        generic_test_temperature_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
    }

    // #[test]
    // fn test_get_data_methods(i2c: Stm32I2c) {
    //     generic_test_get_data_methods(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_get_angle(i2c: Stm32I2c) {
    //     generic_test_get_angle(i2c); // Pass the i2c variable to the inner test function
    // }
}
