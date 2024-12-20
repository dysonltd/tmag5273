#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod cold_start_tests {
    use defmt_rtt as _;
    // use esp_backtrace as _;
    use esp_hal::{
        i2c::master::{AnyI2c, I2c},
        prelude::*,
        Blocking,
    };
    // Optional: A init function which is called before every test
    #[init]
    fn init() -> I2c<'static, Blocking, AnyI2c> {
        let peripherals = esp_hal::init({
            let mut config = esp_hal::Config::default();
            config.cpu_clock = CpuClock::max();
            config
        });
        let i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .with_sda(peripherals.GPIO5)
            .with_scl(peripherals.GPIO6);
        i2c
    }

    #[test]
    fn test_create_tmag5273(i2c: I2c<'static, Blocking, AnyI2c>) {
        let _mag_sensor = tmag5273::TMag5273::new(i2c, tmag5273::types::DeviceVersion::TMAG5273B1)
            .expect("TMag5273::new failed");
    }

    //     // Another example for a conditionally enabled test
    //     #[test]
    //     fn defmt() {
    //         use defmt_rtt as _;
    //         defmt::info!("Hello, defmt!"); // Prints via defmt-rtt to rtt
    //         assert!(true)
    //     }

    //     // Tests can be ignored with the #[ignore] attribute
    //     #[test]
    //     #[ignore]
    //     fn it_works_ignored() {
    //         assert!(false)
    //     }

    //     // A test that fails with a panic
    //     #[test]
    //     fn it_fails1() {
    //         assert!(false)
    //     }

    //     // A test that fails with a returned Err(&str)
    //     #[test]
    //     fn it_fails2() -> Result<(), &'static str> {
    //         Err("It failed because ...")
    //     }

    //     // Tests can be annotated with #[should_panic] if they are expected to panic
    //     #[test]
    //     #[should_panic]
    //     fn it_passes() {
    //         assert!(false)
    //     }

    //     // This test should panic, but doesn't => it fails
    //     #[test]
    //     #[should_panic]
    //     fn it_fails3() {}

    //     // Tests can be annotated with #[timeout(<secs>)] to change the default timeout of 60s
    //     #[test]
    //     #[timeout(10)]
    //     fn it_timeouts() {
    //         loop {} // should run into the 10s timeout
    //     }
}
