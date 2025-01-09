#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests_0 {
    use defmt_rtt as _;
    use stm32f072::initialise::*;
    use tests_common::generic_cold_start_tests::*;

    #[init]
    fn init() -> Stm32I2c {
        initialise()
    }

    #[test]
    fn test_device_id(i2c: Stm32I2c) {
        generic_test_device_id(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_manufacturer_id(i2c: Stm32I2c) {
        generic_test_manufacturer_id(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_registers(i2c: Stm32I2c) {
        generic_test_registers(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_default_i2c_address(i2c: Stm32I2c) {
        generic_test_default_i2c_address(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_magnitude_first_boot(i2c: Stm32I2c) {
        generic_test_get_magnitude_first_boot(i2c); // Pass the i2c variable to the inner test function
    }
}
