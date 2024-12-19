#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use super::*;
    use esp_hal::peripheral::Peripheral;
    use esp_hal::peripherals::Peripherals;
    use esp_hal::prelude::*;
    use esp_hal::delay::Delay;
    use esp_hal::i2c::master::I2c;
    #[init]
    fn init() ->Peripherals {
        esp_println::logger::init_logger(log::LevelFilter::Info);
        let peripherals = esp_hal::init({
            let mut config = esp_hal::Config::default();
            // Configure the CPU to run at the maximum frequency.
            config.cpu_clock = CpuClock::max();
            config
        });
        let delay = Delay::new();
        peripherals 
    }

    #[test]
    fn always_passes(_state: Peripherals) {
        assert!(true);
    }
}
