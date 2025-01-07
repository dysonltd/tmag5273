#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests {
    use defmt_rtt as _;
    use fugit::RateExtU32;
    use rp_pico::{
        hal::{
            self,
            clocks::ClockSource,
            gpio::{self, Pin},
            i2c::I2C,
        },
        pac::{self, I2C1},
    };
    use tests_common::generic_cold_start_tests::*;
    //TODO: Find a way to generify
    type PicoI2c = I2C<
        I2C1,
        (
            Pin<gpio::bank0::Gpio18, gpio::FunctionI2c, gpio::PullUp>,
            Pin<gpio::bank0::Gpio19, gpio::FunctionI2c, gpio::PullUp>,
        ),
    >;
    #[init]
    fn init() -> PicoI2c {
        let mut pac = pac::Peripherals::take().unwrap();
        let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        let clocks = hal::clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();

        // The single-cycle I/O block controls our GPIO pins
        let sio = hal::sio::Sio::new(pac.SIO);

        // Set the pins to their default state
        let pins = gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // Create the I²C drive, using the two pre-configured pins. This will fail
        // at compile time if the pins are in the wrong mode, or if this I²C
        // peripheral isn't available on these pins!

        let i2c_controller = hal::i2c::I2C::new_controller(
            pac.I2C1,
            pins.gpio18.reconfigure(),
            pins.gpio19.reconfigure(),
            400.kHz(),
            &mut pac.RESETS,
            clocks.system_clock.get_freq(),
        );
        i2c_controller
    }

    #[test]
    fn test_device_id(i2c: PicoI2c) {
        generic_test_device_id(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_manufacturer_id(i2c: PicoI2c) {
        generic_test_manufacturer_id(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_registers(i2c: PicoI2c) {
        generic_test_registers(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_default_i2c_address(i2c: PicoI2c) {
        generic_test_default_i2c_address(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_magnitude_first_boot(i2c: PicoI2c) {
        generic_test_get_magnitude_first_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_xyz_thresholds_first_boot(i2c: PicoI2c) {
        generic_test_get_xyz_thresholds_first_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_magnetic_gain(i2c: PicoI2c) {
        generic_test_magnetic_gain(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_magnetic_offset_invalid_at_boot(i2c: PicoI2c) {
        generic_test_magnetic_offset_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_temperature_invalid_at_boot(i2c: PicoI2c) {
        generic_test_temperature_invalid_at_boot(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_data_methods(i2c: PicoI2c) {
        generic_test_get_data_methods(i2c); // Pass the i2c variable to the inner test function
    }

    #[test]
    fn test_get_angle(i2c: PicoI2c) {
        generic_test_get_angle(i2c); // Pass the i2c variable to the inner test function
    }
}
