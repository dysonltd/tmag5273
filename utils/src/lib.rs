use ftdi::Device;
use ftdi_embedded_hal::I2c as FtdiI2c;
use ftdi_embedded_hal::{self as hal};
use std::error::Error;

#[cfg(feature = "rpi")]
use rppal::i2c as PiI2c;

#[cfg(feature = "rpi")]
/// Set up the I2C bus for the Raspberry Pi
pub fn setup_i2c() -> Result<PiI2c::I2c, Box<dyn Error>> {
    let mut i2c = PiI2c::I2c::new()?;
    Ok(i2c)
}

#[cfg(feature = "rpi")]
/// Sets up the I2C bus and the GPIO for the Raspberry Pi The GPIO pin used is the BCM GPIO 4 pin
pub fn setup_i2c_and_gpio() -> Result<(PiI2c::I2c, rppal::gpio::InputPin), Box<dyn Error>> {
    let mut i2c = PiI2c::I2c::new()?;
    let pin = rppal::gpio::Gpio::new()?.get(4)?.into_input();
    Ok((i2c, pin))
}

#[cfg(not(feature = "rpi"))]
/// Set up the I2C bus for the FTDI Interface
pub fn setup_i2c() -> Result<FtdiI2c<Device>, Box<dyn Error>> {
    const BAUDRATE: u32 = 400_000;
    // Change these for your device
    const DEVICE_VID: u16 = 0x0403;
    const DEVICE_PID: u16 = 0x6014;

    let device = ftdi::find_by_vid_pid(DEVICE_VID, DEVICE_PID)
        .interface(ftdi::Interface::A)
        .open()?;
    // Next initialise the HAL with the device and the Baudrate
    let hal = match hal::FtHal::init_freq(device, BAUDRATE) {
        Ok(hal) => hal,
        Err(err) => {
            eprintln!("Failed to initialise HAL: {}", err);
            return Err(Box::new(err));
        }
    };
    // Finally initialise the I2C with the HAL
    let i2c = match hal.i2c() {
        Ok(i2c) => i2c,
        Err(err) => {
            eprintln!("Failed to initialise I2C: {}", err);
            return Err(Box::new(err));
        }
    };
    Ok(i2c)
}

#[cfg(not(feature = "rpi"))]
/// Sets up the I2C bus and the GPIO for the FTDI Interface The GPIO pin used is the CI0 pin
pub fn setup_i2c_and_gpio() -> Result<(FtdiI2c<Device>, hal::InputPin<Device>), Box<dyn Error>> {
    const BAUDRATE: u32 = 400_000;
    // Change these for your device
    const DEVICE_VID: u16 = 0x0403;
    const DEVICE_PID: u16 = 0x6014;

    let device = ftdi::find_by_vid_pid(DEVICE_VID, DEVICE_PID)
        .interface(ftdi::Interface::A)
        .open()?;
    // Next initialise the HAL with the device and the Baudrate
    let hal = match hal::FtHal::init_freq(device, BAUDRATE) {
        Ok(hal) => hal,
        Err(err) => {
            eprintln!("Failed to initialise HAL: {}", err);
            return Err(Box::new(err));
        }
    };

    let pin = match hal.ci0() {
        // The FAKE Interrupt pin
        Ok(pin) => pin,
        Err(err) => {
            eprintln!("Failed to initialise GPIO: {}", err);
            return Err(Box::new(err));
        }
    };

    // Finally initialise the I2C with the HAL
    let i2c = match hal.i2c() {
        Ok(i2c) => i2c,
        Err(err) => {
            eprintln!("Failed to initialise I2C: {}", err);
            return Err(Box::new(err));
        }
    };
    Ok((i2c, pin))
}
