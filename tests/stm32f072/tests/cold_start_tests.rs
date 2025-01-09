#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests {
    use defmt_rtt as _;
    use stm32f0xx_hal::{
        self as hal,
        gpio::{
            gpiob::{PB7, PB8},
            Alternate, AF1,
        },
        pac::I2C1,
    };

    use hal::{i2c::I2c, pac, prelude::*};
    use tests_common::generic_cold_start_tests::*;

    type Stm32I2c = I2c<I2C1, PB8<Alternate<AF1>>, PB7<Alternate<AF1>>>;
    #[init]
    fn init() -> Stm32I2c {
        let p = pac::Peripherals::take().unwrap();
        let mut flash = p.FLASH;
        let mut rcc = p.RCC.configure().freeze(&mut flash);
        let gpiob = p.GPIOB.split(&mut rcc);
        let (scl, sda) = cortex_m::interrupt::free(move |cs| {
            (
                gpiob.pb8.into_alternate_af1(cs),
                gpiob.pb7.into_alternate_af1(cs),
            )
        });

        // Configure I2C with 100kHz rate
        let i2c = Stm32I2c::i2c1(p.I2C1, (scl, sda), 100.khz(), &mut rcc);
        i2c
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

    #[test]
    fn test_get_data_methods(i2c: Stm32I2c) {
        generic_test_get_data_methods(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_angle(i2c: Stm32I2c) {
        generic_test_get_angle(i2c); // Pass the i2c variable to the inner test function
    }
}
