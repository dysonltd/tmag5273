#![no_std]
#![no_main]

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
