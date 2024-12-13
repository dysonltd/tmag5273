#![no_std]
#![no_main]

use esp_hal::i2c::master::I2c;
use tmag5273::types::{DeviceVersion, TMag5273Error};
use tmag5273::TMag5273;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    prelude::*,
};
use embedded_hal::i2c::I2c as I2C_HAL;

/// Simple Main Function to run the example
/// This will set up the I2C bus, create a TMag5273 Sensor, print out some device stats, set up the device and then loop round getting some data
/// The data will be the X, Y and Z axis readings and the temperature
/// The loop will sleep for 100 milliseconds between each reading
#[entry]
fn main() -> ! {
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
    let i2c = I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default(),
    )
    .with_sda(peripherals.GPIO10)
    .with_scl(peripherals.GPIO8);

    let mut mag_sensor = TMag5273::new(i2c, DeviceVersion::TMAG5273B1)
        .unwrap()
        .init_default()
        .unwrap();

    print_device_stats(&mut mag_sensor).unwrap();

    // Loop round and get some data
    loop {
        let data = mag_sensor.get_all_data().unwrap();
        esp_println::println!("data: {:?}", data);
        delay.delay_millis(100);
    }
}

/// Print out some device starts by reading the Device ID and Manufacturer ID, panic if it cant be done
fn print_device_stats<I2C>(mag_sensor: &mut TMag5273<I2C>) -> Result<(), TMag5273Error>
where
    I2C: I2C_HAL,
{
    let device_id = mag_sensor.get_device_id()?;
    esp_println::println!("Device ID: {:?}", device_id);
    let manufacturer_id = mag_sensor.get_manufacturer_id()?;
    esp_println::println!("Manufacturer ID: {:?}", manufacturer_id);
    Ok(())
}
