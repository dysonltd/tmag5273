#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests {
    use defmt_rtt as _;
    use fugit::RateExtU32;
    use rp2040_hal::{
        clocks::ClockSource,
        gpio::{FunctionI2C, Pin},
        i2c::I2C,
        pac::{self, I2C1},
    };
    const XTAL_FREQ_HZ: u32 = 12_000_000u32;
    // use tests_common::generic_cold_start_tests::*;
    #[init]
    fn init() -> I2C<
        I2C1,
        (
            Pin<
                rp2040_hal::gpio::bank0::Gpio18,
                rp2040_hal::gpio::FunctionI2c,
                rp2040_hal::gpio::PullUp,
            >,
            Pin<
                rp2040_hal::gpio::bank0::Gpio19,
                rp2040_hal::gpio::FunctionI2c,
                rp2040_hal::gpio::PullUp,
            >,
        ),
    > {
        let mut pac = pac::Peripherals::take().unwrap();
        let mut watchdog = rp2040_hal::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        let clocks = rp2040_hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();

        // The single-cycle I/O block controls our GPIO pins
        let sio = rp2040_hal::Sio::new(pac.SIO);

        // Set the pins to their default state
        let pins = rp2040_hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // Create the I²C drive, using the two pre-configured pins. This will fail
        // at compile time if the pins are in the wrong mode, or if this I²C
        // peripheral isn't available on these pins!

        let mut i2c_controller = rp2040_hal::I2C::new_controller(
            pac.I2C1,
            pins.gpio18.reconfigure(),
            pins.gpio19.reconfigure(),
            400.kHz(),
            &mut pac.RESETS,
            clocks.system_clock.get_freq(),
        );
        i2c_controller
    }

    // #[test]
    // fn test_is_connected(i2c: I2C<_,_>) {
    //     generic_test_is_connected(i2c); // Pass the i2c variable to the inner test function
    // }
    //     #[test]
    //     fn test_create_tmag5273(i2c: I2c<'static, Blocking, AnyI2c>) {
    //         generic_test_create_tmag5273(i2c); // Pass the i2c variable to the inner test function
    //     }
    //     #[test]
    //     fn test_set_reset_device_config_1_register(i2c: I2c<'static, Blocking, AnyI2c>) {
    //         generic_test_set_reset_device_config_1_register(i2c); // Pass the i2c variable to the inner test function
    //     }
    //     #[test]
    //     fn test_set_reset_device_config_2_register(i2c: I2c<'static, Blocking, AnyI2c>) {
    //         generic_test_reset_device_config_2_register(i2c); // Pass the i2c variable to the inner test function
    //     }
    //     #[test]
    //     fn test_set_reset_i2c_address_register(i2c: I2c<'static, Blocking, AnyI2c>) {
    //         generic_test_set_reset_i2c_address_register(i2c); // Pass the i2c variable to the inner test function
    //     }
    //     #[test]
    //     fn test_set_reset_int_config_1_register(i2c: I2c<'static, Blocking, AnyI2c>) {
    //         generic_test_set_reset_int_config_1_register(i2c); // Pass the i2c variable to the inner test function
    //     }
    //     #[test]
    //     fn test_set_reset_sensor_config_1_register(i2c: I2c<'static, Blocking, AnyI2c>) {
    //         generic_test_set_reset_sensor_config_1_register(i2c); // Pass the i2c variable to the inner test function
    //     }
    //     #[test]
    //     fn test_set_reset_sensor_config_2_register(i2c: I2c<'static, Blocking, AnyI2c>) {
    //         generic_test_set_reset_sensor_config_2_register(i2c); // Pass the i2c variable to the inner test function
    //     }
    //     #[test]
    //     fn test_set_reset_t_config_register(i2c: I2c<'static, Blocking, AnyI2c>) {
    //         generic_test_set_reset_t_config_register(i2c); // Pass the i2c variable to the inner test function
    //     }
}
