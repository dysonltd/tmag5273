#![no_std]
#![no_main]

use embedded_hal::i2c::{I2c, SevenBitAddress};
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::{
    clock::ClockControl, gpio::Io, i2c::I2C, peripherals::Peripherals, prelude::*,
    system::SystemControl,
};
use tmag5273::types::{DeviceVersion, TMag5273Error};
use tmag5273::TMag5273;
use crate::traits::{BitFieldDeviceConfiguration, ByteFieldDeviceConfiguration};

/// Simple Main Function to run the example
/// This will set up the I2C bus, create a TMag5273 Sensor, print out some device stats, set up the device and then loop round getting some data
/// The data will be the X, Y and Z axis readings and the temperature
/// The loop will sleep for 100 milliseconds between each reading
#[entry]
fn main() -> ! {
    esp_println::logger::init_logger(log::LevelFilter::Info);
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    esp_println::println!("Running 1 Basic Readings!");
    // Set up your I2C
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio10,
        io.pins.gpio8,
        400.kHz(),
        &clocks,
    );

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
    I2C: I2c<SevenBitAddress>,
{
    let device_id = mag_sensor.get_device_id()?;
    esp_println::println!("Device ID: {:?}", device_id);
    let manufacturer_id = mag_sensor.get_manufacturer_id()?;
    esp_println::println!("Manufacturer ID: {:?}", manufacturer_id);
    Ok(())
}
