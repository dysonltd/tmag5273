use embedded_hal::i2c::{I2c, SevenBitAddress};
use std::error::Error;
use tmag5273::{
    registers::*,
    types::{DeviceVersion, TMag5273Error},
    DeviceConfig, SensorConfig, TMag5273,
};
use utils::setup_i2c;

/// Simple Main Function to run the example
/// This will set up the I2C bus, create a TMag5273 Sensor, print out some device stats, set up the device and then loop round getting some data
/// The data will be the angle of the sensor
fn main() -> Result<(), Box<dyn Error>> {
    println!("Running Example 3 Angle Calculations!");

    let i2c = setup_i2c()?; // Set up the I2C Bus

    let mut mag_sensor = TMag5273::new(i2c, DeviceVersion::TMAG5273B1)?.init_default()?;

    print_device_stats(&mut mag_sensor)?;

    setup_device(&mut mag_sensor)?;

    // Loop Round and get some data
    loop {
        match mag_sensor.get_angle() {
            Ok(data) => println!("Angle: {data:?}, "),
            Err(err) => eprintln!("Failed to get Angle data: {err:?}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// Set up the device with the following settings:
fn setup_device<I2C>(mag_sensor: &mut TMag5273<I2C>) -> Result<(), TMag5273Error>
where
    I2C: I2c<SevenBitAddress>,
{
    let device_config = mag_sensor.get_device_config()?;
    mag_sensor.set_device_config(DeviceConfig {
        conv_avg: ConversionAverage::X32,
        ..device_config
    })?;
    let sensor_config = mag_sensor.get_sensor_config()?;
    mag_sensor.set_sensor_config(SensorConfig {
        mag_channel: MagneticChannel::XYX,
        angle: Angle::XY,
        ..sensor_config
    })?;
    Ok(())
}

fn print_device_stats<I2C>(mag_sensor: &mut TMag5273<I2C>) -> Result<(), TMag5273Error>
where
    I2C: I2c<SevenBitAddress>,
{
    let device_id = mag_sensor.get_device_id()?;
    println!("Device ID: {device_id:?}");
    let manufacturer_id = mag_sensor.get_manufacturer_id()?;
    println!("Manufacturer ID: {manufacturer_id:?}");
    Ok(())
}
