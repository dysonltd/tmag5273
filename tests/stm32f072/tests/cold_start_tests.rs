#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests {
    use defmt_rtt as _;
    use stm32f0xx_hal as hal;

    use hal::{i2c::I2c, pac, prelude::*};
    // use tests_common::generic_cold_start_tests::*;

    #[init]
    fn init() -> () {
        if let Some(p) = pac::Peripherals::take() {
            cortex_m::interrupt::free(move |cs| {
                let mut flash = p.FLASH;
                let mut rcc = p.RCC.configure().freeze(&mut flash);

                let gpiob = p.GPIOB.split(&mut rcc);

                // Configure pins for I2C
                let sda = gpiob.pb7.into_alternate_af1(cs);
                let scl = gpiob.pb8.into_alternate_af1(cs);

                // Configure I2C with 100kHz rate
                let mut i2c = I2c::i2c1(p.I2C1, (scl, sda), 100.khz(), &mut rcc);

                let mut _devices = 0;
                // I2C addresses are 7-bit wide, covering the 0-127 range
                for add in 0..=127 {
                    // The write method sends the specified address and checks for acknowledgement;
                    // if no ack is given by the slave device the result is Err(), otherwise Ok()
                    // Since we only care for an acknowledgement the data sent can be empty
                    if i2c.write(add, &[]).is_ok() {
                        _devices += 1;
                    }
                }

                // Here the variable "_devices" counts how many i2c addresses were a hit
            });
        }
    }

    #[test]
    fn test() {
        assert_eq!(true, true)
    }
    // #[test]
    // fn test_device_id(i2c: I2c) {
    //     generic_test_device_id(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_manufacturer_id(i2c: I2c) {
    //     generic_test_manufacturer_id(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_registers(i2c: I2c) {
    //     generic_test_registers(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_default_i2c_address(i2c: I2c) {
    //     generic_test_default_i2c_address(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_get_magnitude_first_boot(i2c: I2c) {
    //     generic_test_get_magnitude_first_boot(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_get_xyz_thresholds_first_boot(i2c: I2c) {
    //     generic_test_get_xyz_thresholds_first_boot(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_magnetic_gain(i2c: I2c) {
    //     generic_test_magnetic_gain(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_magnetic_offset_invalid_at_boot(i2c: I2c) {
    //     generic_test_magnetic_offset_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_temperature_invalid_at_boot(i2c: I2c) {
    //     generic_test_temperature_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_get_data_methods(i2c: I2c) {
    //     generic_test_get_data_methods(i2c); // Pass the i2c variable to the inner test function
    // }

    // #[test]
    // fn test_get_angle(i2c: I2c) {
    //     generic_test_get_angle(i2c); // Pass the i2c variable to the inner test function
    // }
}
