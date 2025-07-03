use embedded_hal::i2c::{I2c, SevenBitAddress};
use std::error::Error;
use tmag5273::{
    registers::*,
    types::{Axis, DeviceVersion, TMag5273Error},
    InterruptConfig, TMag5273,
};
use utils::setup_i2c_and_gpio;

/// Simple Main Function to run the example
/// This will set up the Hall effect Sensor to send out a high on the interrupt pin when the sensor has finished its scan
/// The loop will then get the X, Y and Z axis readings
/// The loop constantly polls the Interrupt pin
fn main() -> Result<(), Box<dyn Error>> {
    println!("Running Example 2 Interrupts!");

    let (i2c, mut pin) = setup_i2c_and_gpio()?; // Set up your I2C and gpio pin

    let mut mag_sensor = TMag5273::new(i2c, DeviceVersion::TMAG5273B1)?.init_default()?;
    print_device_stats(&mut mag_sensor)?;

    //Set up the device for this example
    if let Err(err) = setup_device(&mut mag_sensor) {
        panic!("Failed to set up device: {err:?}");
    }

    loop {
        if check_pin(&mut pin) {
            println!("Sensor has Finished its scan!!");
            //Get the X Threshold and print it out
            match mag_sensor.get_mag_threshold(Axis::X) {
                Ok(x_threshold) => println!("X Threshold: {x_threshold:?}"),
                Err(err) => println!("Failed to get X Threshold: {err:?}"),
            }
            println!("Sensor has now finished its scan!!");
            // Get the data
            match mag_sensor.get_all_data() {
                Ok(data) => println!("Data: {data:?}"),
                Err(err) => println!("Failed to get data: {err:?}"),
            }
        }
    }
}

fn check_pin<Pin>(pin: &mut Pin) -> bool
where
    Pin: embedded_hal::digital::InputPin,
{
    match pin.is_low() {
        Ok(is_low) => is_low,
        Err(err) => {
            eprintln!("Failed to read pin: {err:?}");
            false
        }
    }
}

fn setup_device<I2C>(mag_sensor: &mut TMag5273<I2C>) -> Result<(), TMag5273Error>
where
    I2C: I2c<SevenBitAddress>,
{
    mag_sensor.set_interrupts(InterruptConfig {
        interrupt_mode: InterruptMode::INTInterrupt,
        int_pin_mode: INTPinMode::Pulsed,
        threshold_interrupt_enabled: true,
        ..Default::default()
    })?;

    //Set X Threshold
    let threshold = 5.0;
    mag_sensor.set_mag_threshold(threshold, Axis::X)?; // 5mT
    println!("Threshold Set to: {threshold:?}");

    // Get Threshold
    let applied_threshold = mag_sensor.get_mag_threshold(Axis::X)?;
    println!("Applied Threshold: {applied_threshold:?}");
    assert_eq!(applied_threshold, threshold);

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
