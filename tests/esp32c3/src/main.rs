#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};
use log::info;
#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let delay = Delay::new();
    esp_println::logger::init_logger_from_env(); // # prints to jtag/uart0 !

    loop {
        info!("Hello world!");
        delay.delay(500.millis());
    }
}
