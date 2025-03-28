#![no_std]
/// This module is a simple setup method and type declarations that can be shared across the tests, reducing boiler plate code.
pub mod initialise {
    use defmt_rtt as _;
    use esp_hal::{clock::CpuClock, i2c::master::I2c, Blocking};
    pub type EspI2c = I2c<'static, Blocking>;

    pub fn initialise() -> EspI2c {
        let peripherals = esp_hal::init({
            let mut config = esp_hal::Config::default();
            config.cpu_clock = CpuClock::max();
            config
        });
        let i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .unwrap()
            .with_sda(peripherals.GPIO5)
            .with_scl(peripherals.GPIO6);
        i2c
    }
}
