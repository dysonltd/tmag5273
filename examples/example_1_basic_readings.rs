use embedded_hal::i2c::{I2c, SevenBitAddress};
use std::error::Error;
use tmag5273::types::{DeviceVersion, TMag5273Error};
use tmag5273::TMag5273;
use utils::setup_i2c;

/// Simple Main Function to run the example
/// This will set up the I2C bus, create a TMag5273 Sensor, print out some device stats, set up the device and then loop round getting some data
/// The data will be the X, Y and Z axis readings and the temperature
/// The loop will sleep for 100 milliseconds between each reading
fn main() -> Result<(), Box<dyn Error>> {
    println!("Running 1 Basic Readings!");

    let i2c = setup_i2c()?; // Set up the I2C Bus, uses feature flags for different implementations

    let mut mag_sensor = TMag5273::new(i2c, DeviceVersion::TMAG5273B1)?.init_default()?;

    print_device_stats(&mut mag_sensor)?;

    // Loop Round and get some data
    loop {
        let data = mag_sensor.get_all_data()?;
        println!("data: {:?}", data);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn print_device_stats<I2C>(mag_sensor: &mut TMag5273<I2C>) -> Result<(), TMag5273Error>
where
    I2C: I2c<SevenBitAddress>,
{
    let device_id = mag_sensor.get_device_id()?;
    println!("Device ID: {:?}", device_id);
    let manufacturer_id = mag_sensor.get_manufacturer_id()?;
    println!("Manufacturer ID: {:?}", manufacturer_id);
    Ok(())
}
