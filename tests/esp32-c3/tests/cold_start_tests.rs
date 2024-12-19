#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use super::*;
    use embedded_hal::i2c::I2c as I2C_HAL;
    #[init]
    fn init() -> I2C_HAL {
        esp_println::logger::init_logger(log::LevelFilter::Info);
        let peripherals = esp_hal::init({
            let mut config = esp_hal::Config::default();
            // Configure the CPU to run at the maximum frequency.
            config.cpu_clock = CpuClock::max();
            config
        });
        let delay = Delay::new();
        esp_println::println!("Running 1 Basic Readings!");
        // Set up your I2C
        let i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .with_sda(peripherals.GPIO5)
            .with_scl(peripherals.GPIO6);

        // The init function can return some state, which can be consumed by the testcases
        i2c
    }

    #[test]
    fn always_passes(_state: I2C_HAL) {
        assert!(true);
    }
}
