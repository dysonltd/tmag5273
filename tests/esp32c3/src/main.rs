#![no_std]
#![no_main]

/*
This file is a placeholder for the embedded-test framework to run tests on the ESP32C3. All tests can be found
in the tests/esp32c3/tests directory.
*/
use defmt_rtt as _;
use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};
#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let delay = Delay::new();

    loop {
        defmt::info!("Hello World!");
        delay.delay(500.millis());
    }
}
