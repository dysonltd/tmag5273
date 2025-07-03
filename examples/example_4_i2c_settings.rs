use embedded_hal::i2c::{I2c, SevenBitAddress};
use std::error::Error;
use tmag5273::{
    registers::*,
    types::{DeviceVersion, TMag5273Error},
    TMag5273,
};
use utils::setup_i2c;

/// Simple Main Function to run the example
/// This will set up the I2C bus, create a TMag5273 Sensor, print out some device stats, set up the device and then loop round getting some data
/// The data will be the angle of the sensor
fn main() -> Result<(), Box<dyn Error>> {
    println!("Running Example 4 I2C Settings!");
    let new_i2c_address: SevenBitAddress = 0x23;

    let i2c = setup_i2c()?;

    let mut mag_sensor = TMag5273::new(i2c, DeviceVersion::TMAG5273B1)?.init_default()?;

    print_device_stats(&mut mag_sensor)?;

    //Original I2C Address
    let register: I2cAddressRegister = mag_sensor.get_config_register()?;

    let i2c_address = register.i2c_address().value();
    println!("Original I2C Address: {i2c_address:x}");
    println!("Setting I2C Address to: {new_i2c_address:x}");
    let register_update = register
        .with_i2c_address(arbitrary_int::u7::new(new_i2c_address))
        .with_i2c_address_update_enabled(true);

    mag_sensor.set_config_register(register_update)?;

    // Drop the TMag5273 instance
    drop(mag_sensor);

    // Set everything up again with the new address of the device
    let i2c = setup_i2c()?;
    println!(
        "Getting Address from Sensor on new Address {new_i2c_address:x}"
    );
    let mut tmag5273 = TMag5273::new_with_address(i2c, new_i2c_address, DeviceVersion::TMAG5273A1)?;

    let register: I2cAddressRegister = tmag5273.get_config_register()?;
    println!("New I2C Address: {:x}", register.i2c_address().value());
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
