/*!
 * # TMAG5273 Rust Driver
 *
 * ## Summary
 * This is a platform agnostic Rust Driver for the TMAG52732 3 Axis I2C Hall effect Sensor by Texas Instruments. The driver is based on the [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits.
 *
 * The driver has been tested with the following breakout Boards:
 * - [TMAG5273 Sparkfun Breakout Board](https://www.sparkfun.com/products/23880)
 *
 * and the following platforms:
 *
 * - [Raspberry Pi 4](https://thepihut.com/products/raspberry-pi-4-model-b?srsltid=AfmBOoprSW4fNwFwrEr96zq88-m8EI3SJZb1MOjiibxcs9fKWCdoMcZD) Running [Raspberry Pi OS](https://www.raspberrypi.com/software/)
 * - [Apple M1 Macbook Pro](https://www.apple.com/uk/mac/compare/?modelList=MacBook-Pro-16-2021,MacBook-Air-M2,MacBook-Air-M3-15) on Sonoma 14.6.1
 * - Linux Desktop on Ubuntu
 * - [ESP32 Rust Board](https://github.com/esp-rs/esp-rust-board) on esp-hal 0.19.0
 *
 *
 * ## Examples
 *
 * Examples on how to use the driver across multiple platforms can be found in the examples directory of the repo.
 *
 *
 * As an application developer you will spend most of your time working with the TMAG5273 struct.
 * initialising of the device can be done using the `init_default` method and configuration can be done using the methods outlined in config.rs. Essentially you will work
 * with the following structs:
 *
 * - DeviceConfig
 * - SensorConfig
 * - InterruptConfig
 *  
 * However for fine grained control of the device you can set and configure
 * the raw registers outlined in registers module.
 *
 * ## Fault Handling
 *
 * The goal of this library is to prioritise robustness. To achieve this, certain design choices were made that introduce minimal overhead in I2C communication. Traditionally,
 * I2C sensors provide getter methods for reading data registers, and users typically access these registers without verifying the validity of the data. To address this issue,
 * our library first reads the relevant configuration register and raises a WrongMode error if the sensor is in an incorrect state. Only then does it proceed to retrieve the
 * requested data, preventing potential system corruption. While this approach introduces a slight performance overhead, it significantly enhances the overall reliability and
 * robustness of the code.
 */
#![no_std]
mod config;

pub mod registers;
pub mod types;
pub use config::*;

use embedded_hal::i2c::{I2c, SevenBitAddress};
use registers::*;
use types::{Axis, DeviceVersion, MagneticChannelOffset, TMag5273ChannelData, TMag5273Error};

/// Value found in the device ID register
pub const MANUFACTURER_ID_VALUE: u16 = 0x5449;

pub struct TMag5273<I2C> {
    device_version: DeviceVersion,
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: SevenBitAddress,
}

impl<I2C> TMag5273<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Create an instance of the TMag5273xx device.
    ///
    /// The device will be checked for connectivity by reading the manufacturer ID.
    ///
    /// ## Example
    ///
    /// ```Rust
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let i2c = I2c::new(i2c_bus);
    ///     let mut mag5273 = TMag5273::new(i2c, Version::TMAG5273B1)?.init_default()?;
    ///
    /// }
    /// ```
    pub fn new(i2c: I2C, version: DeviceVersion) -> Result<TMag5273<I2C>, TMag5273Error> {
        TMag5273::new_with_address(i2c, version.get_default_address(), version)
    }

    /// Create an instance of a TMag5273xx device with a specific address.
    pub fn new_with_address(
        i2c: I2C,
        address: SevenBitAddress,
        version: DeviceVersion,
    ) -> Result<TMag5273<I2C>, TMag5273Error> {
        let mut device = TMag5273 {
            i2c,
            address,
            device_version: version,
        };
        device.get_manufacturer_id()?;
        device.is_correct_device_version()?;
        Ok(device)
    }

    /// Check if the device is connected.
    pub fn is_connected(&mut self) -> bool {
        self.get_manufacturer_id().is_ok()
    }

    /// Get the manufacturer ID.
    pub fn get_manufacturer_id(&mut self) -> Result<u16, TMag5273Error> {
        let mut data: [u8; 2] = [0x00, 0x00];
        let register_address = TMAG5273Register::ManufacturerIdLsb.into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;

        let manufacturer_id = u16::from_le_bytes(data);
        match manufacturer_id != MANUFACTURER_ID_VALUE {
            true => Err(TMag5273Error::WrongDevice),
            false => Ok(manufacturer_id),
        }
    }

    /// Returns the Device ID of the device, otherwise an Error if the device is not connected or the register is malformed.
    pub fn get_device_id(&mut self) -> Result<DeviceId, TMag5273Error> {
        let device_id_register = self.get_config_register::<DeviceIdRegister>()?;
        let device_id = device_id_register
            .device_id()
            .map_err(|_| TMag5273Error::MalformedRegister)?;
        Ok(device_id)
    }

    /// Set the threshold for the device. The threshold is a floating point value, which is converted to a
    /// raw value and set on the device.
    /// The threshold is set for the X, Y and Z axis.
    /// The range is calculated based on the range of the device.
    ///
    /// ### Arguments
    ///
    /// * `threshold` - The threshold value in mT
    /// * `axis` - The axis to set the threshold for
    pub fn set_mag_threshold(&mut self, threshold: f32, axis: Axis) -> Result<(), TMag5273Error> {
        let config = self.get_config_register::<SensorConfig2Register>()?;
        let (register, range) = match axis {
            Axis::X => (TMAG5273Register::XThrConfig, config.xy_range()),
            Axis::Y => (TMAG5273Register::YThrConfig, config.xy_range()),
            Axis::Z => (TMAG5273Register::ZThrConfig, config.z_range()),
        };
        // check that the threshold is within the range of the device
        if threshold > range.get_range(self.device_version)
            || threshold < -range.get_range(self.device_version)
        {
            return Err(TMag5273Error::OutOfRange);
        }
        let threshold_normalized =
            (threshold / range.get_range(self.device_version)).clamp(-1.0, 1.0);
        let threshold_raw = (threshold_normalized * 128.0) as i8;
        let write = [register.into(), threshold_raw.to_le_bytes()[0]];
        self.i2c.write(self.address, &write)?;
        Ok(())
    }

    /// Get the threshold for the device. The threshold is a floating point value, which is converted from a
    /// raw value sent from the device.
    /// The range is calculated based on the range that the device is currently configured for.
    ///
    /// Output in mT.
    ///
    pub fn get_mag_threshold(&mut self, axis: Axis) -> Result<f32, TMag5273Error> {
        let config = self.get_config_register::<SensorConfig2Register>()?;
        let (register, range) = match axis {
            Axis::X => (TMAG5273Register::XThrConfig, config.xy_range()),
            Axis::Y => (TMAG5273Register::YThrConfig, config.xy_range()),
            Axis::Z => (TMAG5273Register::ZThrConfig, config.z_range()),
        };
        let threshold_normalized = {
            let mut buf: [u8; 1] = [0x00];
            let register_address = register.into();
            self.i2c
                .write_read(self.address, &[register_address], &mut buf)?;
            let threshold_raw = i8::from_le_bytes(buf); // always one byte (-128 to 127)
            threshold_raw as f32 / 128.0 // convert to -1.0 to 1.0
        };
        Ok(threshold_normalized * range.get_range(self.device_version))
    }

    pub fn set_temperature_threshold() {
        // TODO: Implement see datasheet Page 29
        unimplemented!()
    }
    pub fn get_temperature_threshold() {
        // TODO: Implement see datasheet Page 29
        unimplemented!()
    }

    /// Set the magnetic gain for the device.
    /// Gain value mapped 0 - 255 = 0 - 100%
    pub fn set_magnetic_gain(&mut self, gain: u8) -> Result<(), TMag5273Error> {
        let register_address = TMAG5273Register::MagGainConfig.into();
        self.i2c.write(self.address, &[register_address, gain])?;
        Ok(())
    }

    /// Get the magnetic gain for the device.
    /// Gain value mapped 0 - 255 = 0 - 100%
    pub fn get_magnetic_gain(&mut self) -> Result<u8, TMag5273Error> {
        let mut data: [u8; 1] = [0x00];
        let register_address = TMAG5273Register::MagGainConfig.into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;
        Ok(data[0])
    }

    /// Set the magnetic offset for the device.
    ///
    /// NOTE: the angle must be set first.
    pub fn set_magnetic_offset(
        &mut self,
        offset: f32,
        mag_offset: MagneticChannelOffset,
    ) -> Result<(), TMag5273Error> {
        let _register_address: TMAG5273Register = mag_offset.into();
        let config: SensorConfig2Register = self.get_config_register()?;
        let range = match config.angle() {
            Angle::Disabled => return Err(TMag5273Error::WrongMode),
            Angle::YZ | Angle::XZ => config.z_range(),
            Angle::XY => config.xy_range(),
        }
        .get_range(self.device_version);
        // make sure offset is within full range
        let full_range = range * 2.0;
        if offset > full_range || offset < -full_range {
            return Err(TMag5273Error::OutOfRange);
        }
        const COEFFICIENT: f32 = 2_i16.pow(12) as f32; // 2048 from the datasheet
        let _raw_offset = (COEFFICIENT * offset) / full_range;
        unimplemented!("Offset doesn't work yet");
        // let write = register_address.into();
        // self.i2c
        //     .write(self.address, &[write, raw_offset.to_le_bytes()[0]])?;
        // Ok(())
    }

    /// Get the magnetic offset for the device.
    pub fn get_magnetic_offset(
        &mut self,
        mag_offset: MagneticChannelOffset,
    ) -> Result<u8, TMag5273Error> {
        let register_address: TMAG5273Register = mag_offset.into();
        let config: SensorConfig2Register = self.get_config_register()?;
        let _range = match config.angle() {
            Angle::Disabled => return Err(TMag5273Error::WrongMode),
            Angle::YZ | Angle::XZ => config.z_range(),
            Angle::XY => config.xy_range(),
        }
        .get_range(self.device_version);
        let _data = {
            let mut buf: [u8; 1] = [0x00];
            let register_address = register_address.into();
            self.i2c
                .write_read(self.address, &[register_address], &mut buf)?;
            buf[0] // always one byte
        };
        unimplemented!("Offset doesn't work yet");
        // if mag_offset == MagOffset::Offset1 {
        //     Ok(((data as f32) * (xy_range) / 2048.0) as u8)
        // } else {
        //     let angle_en: AngleEn = config.angle_en().into();
        //     let z_range = config.z_range().get_range(self.version);
        //     Ok(((data as f32) * (range) / 2048.0) as u8)
        // }
    }

    /// Get the temperature of the device in degrees Celsius. This will
    /// throw an error if the temperature channel is not enabled.
    pub fn get_temperature(&mut self) -> Result<f32, TMag5273Error> {
        // Check if channel is enabled, throw error otherwise
        self.check_temp_channel()?;
        let mut data: [u8; 2] = [0x00; 2];
        let register_address = TMAG5273Register::TMsbResult.into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;
        Ok(Self::convert_temp(data))
    }

    /// Gets the specific axis channel data. This will throw an error if the channel is not enabled.
    pub fn get_mag_data(&mut self, axis: Axis) -> Result<f32, TMag5273Error> {
        let configs: SensorConfigRegisters = self.get_dual_config_register()?;
        let config2 = configs.sensor_config2();
        // Verify the axis is enabled
        let mag_channel = configs.sensor_config1().mag_channel();

        // Guard against fault states
        match mag_channel {
            MagneticChannel::Default => return Err(TMag5273Error::WrongMode),
            MagneticChannel::Reserved1
            | MagneticChannel::Reserved2
            | MagneticChannel::Reserved3
            | MagneticChannel::Reserved4 => return Err(TMag5273Error::MalformedRegister),
            _ => (),
        }
        // Verify that the axis we want is enabled
        let (register, range) = match axis {
            Axis::X => match mag_channel {
                MagneticChannel::Y
                | MagneticChannel::Z
                | MagneticChannel::YZ
                | MagneticChannel::YZY => return Err(TMag5273Error::ChannelDisabled),
                _ => (TMAG5273Register::XMsbResult, config2.xy_range()),
            },
            Axis::Y => match mag_channel {
                MagneticChannel::X
                | MagneticChannel::Z
                | MagneticChannel::XZ
                | MagneticChannel::XZX => return Err(TMag5273Error::ChannelDisabled),
                _ => (TMAG5273Register::YMSBResult, config2.xy_range()),
            },
            Axis::Z => match mag_channel {
                MagneticChannel::X
                | MagneticChannel::Y
                | MagneticChannel::XY
                | MagneticChannel::XYX => return Err(TMag5273Error::ChannelDisabled),
                _ => (TMAG5273Register::ZMSBResult, config2.z_range()),
            },
        };

        let range = range.get_range(self.device_version);
        let mut data: [u8; 2] = [0x00; 2];
        let register_address = register.into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;
        Ok(Self::convert_magnetism(axis, data, range))
    }

    /// Get all the axis data from the device, alongside the temperature of the device
    /// The data is returned as a TMag5273Data struct. If the correct Channels are not enabled,
    /// an error is thrown.
    pub fn get_all_data(&mut self) -> Result<TMag5273ChannelData, TMag5273Error> {
        let configs: SensorConfigRegisters = self.get_dual_config_register()?;
        let config2 = configs.sensor_config2();
        // Only valid in XYZ mode
        if configs.sensor_config1().mag_channel() != MagneticChannel::XYZ {
            return Err(TMag5273Error::ChannelDisabled);
        }
        // Verify the temperature channel is enabled
        self.check_temp_channel()?;

        // Full Data Read
        let mut data: [u8; 8] = [0x00; 8];
        let register_address = TMAG5273Register::TMsbResult.into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;

        let xy_range = config2.xy_range().get_range(self.device_version);
        let z_range = config2.z_range().get_range(self.device_version);

        let temp = Self::convert_temp([data[0], data[1]]);
        let x = Self::convert_magnetism(Axis::X, [data[2], data[3]], xy_range);
        let y = Self::convert_magnetism(Axis::Y, [data[4], data[5]], xy_range);
        let z = Self::convert_magnetism(Axis::Z, [data[6], data[7]], z_range);
        Ok(TMag5273ChannelData { x, y, z, temp })
    }

    /// Gets the measured/calculated angle in degrees of the enabled channel
    /// This will throw an error if the channels are not enabled.
    pub fn get_angle(&mut self) -> Result<f32, TMag5273Error> {
        // Check if the channel is enabled
        let config: SensorConfig2Register = self.get_config_register()?;
        if config.angle() == Angle::Disabled {
            return Err(TMag5273Error::ChannelDisabled);
        }
        let mut data = [0x00; 2];
        let register_address = TMAG5273Register::AngleResultMSB.into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;

        // The angle is calculated as follows:
        //  x x x x x x x x   x x x x x x x x
        //  |____0x19_____|   |_____0x1A____|
        //        |_________________| |_____|
        //                 ^             ^ Divide by 16 to give angle fraction in degrees
        //                 '- Angle integral in degrees
        let angle_reg = i16::from_be_bytes(data);
        let dec_value = (data[1] & 0b1111) as f32 / 16.0;
        let angle_val = (angle_reg >> 4) as f32;
        Ok(angle_val + dec_value)
    }

    /// Returns the resultant vector magnitude (during the angle measurement) result. This value should be consistent during 360 degrees measurements.
    pub fn get_magnitude(&mut self) -> Result<u8, TMag5273Error> {
        let mut data: [u8; 1] = [0x00];
        let register_address = TMAG5273Register::MagnitudeResult.into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;
        Ok(data[0])
    }

    /// Initialise the device with the default settings.
    pub fn init_default(mut self) -> Result<Self, TMag5273Error> {
        // Set the Mag Channels to be enabled
        let sensor_config_1_register: SensorConfig1Register = self
            .get_config_register::<SensorConfig1Register>()?
            .with_sleep_time(SleepTime::Ms20000)
            .with_mag_channel(MagneticChannel::XYZ);
        self.set_config_register(sensor_config_1_register)?;

        // Set the Sensor Config 2 Register
        let sensor_config_2_register: SensorConfig2Register = self
            .get_config_register::<SensorConfig2Register>()?
            .with_xy_range(Range::High)
            .with_z_range(Range::High);
        self.set_config_register(sensor_config_2_register)?;

        // Set the Temperature Sensor to be enabled
        let t_config_register: TConfigRegister = self
            .get_config_register::<TConfigRegister>()?
            .with_temperature_channel_enabled(true);
        self.set_config_register(t_config_register)?;

        // Set the Operating Mode
        let device_config2_register: DeviceConfig2Register = self
            .get_config_register::<DeviceConfig2Register>()?
            .with_operating_mode(OperatingMode::ContinuousMeasure);
        self.set_config_register(device_config2_register)?;

        Ok(self)
    }
}
impl<I2C> TMag5273<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Convert the magnetism data from the device to a floating point value.
    fn convert_magnetism(axis: Axis, data: [u8; 2], range: f32) -> f32 {
        let data = i16::from_be_bytes(data) as f32;
        let value = (data * range) / 32768.0;
        if axis == Axis::X {
            -value
        } else {
            value
        }
    }

    /// Convert the temperature data from the device to a floating point value in (degC).
    fn convert_temp(data: [u8; 2]) -> f32 {
        const TSENSE_T0: f32 = 25.0; // Reference temperature for TADC_T0
        const TADC_T0: f32 = 17508.0; // Temp result in decimal value (from 16-bit format)
        const TADC_RES: f32 = 60.1; // Temperature sensing resolution (in 16-bit format)
        let raw_temp = u16::from_be_bytes(data) as f32;
        TSENSE_T0 + ((raw_temp - TADC_T0) / TADC_RES)
    }

    /// Checks the temp channel is enabled
    fn check_temp_channel(&mut self) -> Result<(), TMag5273Error> {
        let register_address = TMAG5273Register::TConfig.into();
        let mut data: [u8; 1] = [0x00];
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;
        let tch_en = TConfigRegister::new_with_raw_value(data[0]).temperature_channel_enabled();

        if tch_en {
            Ok(())
        } else {
            Err(TMag5273Error::ChannelDisabled)
        }
    }

    /// Checks if the connect device has the correct hardware version for what the user has chosen
    fn is_correct_device_version(&mut self) -> Result<(), TMag5273Error> {
        match self.get_device_id()? != self.device_version.get_device_id() {
            true => Err(TMag5273Error::WrongDevice),
            false => Ok(()),
        }
    }
}
