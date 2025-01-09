#![no_std]
#![no_main]

pub mod initialise {
    use embassy_stm32::{
        bind_interrupts,
        dma::NoDma,
        i2c::{self, I2c},
        peripherals,
        time::Hertz,
        Config,
    };
    pub type Stm32I2c =
        embassy_stm32::i2c::I2c<'static, embassy_stm32::peripherals::I2C1, NoDma, NoDma>;
    pub fn initialise() -> Stm32I2c {
        bind_interrupts!(struct Irqs {
            I2C1 => i2c::EventInterruptHandler<peripherals::I2C1>, i2c::ErrorInterruptHandler<peripherals::I2C1>;
        });
        let config = Config::default();
        let p = embassy_stm32::init(config);
        let i2c = I2c::new(
            p.I2C1,
            p.PB8,
            p.PB9,
            Irqs,
            NoDma,
            NoDma,
            Hertz(400_000),
            Default::default(),
        );
        i2c
    }
}

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests_0 {
    use crate::initialise::*;
    use defmt_rtt as _;
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

// #[cfg(test)]
// #[embedded_test::tests]
// mod cold_start_tests_1 {
//     use crate::initialise::*;
//     use defmt_rtt as _;
//     use tests_common::generic_cold_start_tests::*;

//     #[init]
//     fn init() -> Stm32I2c {
//         initialise()
//     }

//     #[test]
//     fn test_get_xyz_thresholds_first_boot(i2c: Stm32I2c) {
//         generic_test_get_xyz_thresholds_first_boot(i2c); // Pass the i2c variable to the inner test function
//     }

//     #[test]
//     fn test_magnetic_gain(i2c: Stm32I2c) {
//         generic_test_magnetic_gain(i2c); // Pass the i2c variable to the inner test function
//     }

//     #[test]
//     fn test_magnetic_offset_invalid_at_boot(i2c: Stm32I2c) {
//         generic_test_magnetic_offset_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
//     }

//     #[test]
//     fn test_temperature_invalid_at_boot(i2c: Stm32I2c) {
//         generic_test_temperature_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
//     }

//     #[test]
//     fn test_get_data_methods(i2c: Stm32I2c) {
//         generic_test_get_data_methods(i2c); // Pass the i2c variable to the inner test function
//     }

//     #[test]
//     fn test_get_angle(i2c: Stm32I2c) {
//         generic_test_get_angle(i2c); // Pass the i2c variable to the inner test function
//     }
// }
