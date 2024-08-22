use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::{registers::*, TMag5273, TMag5273Error};

#[derive(Default, Debug)]
pub struct InterruptConfig {
    /// Mask INT pin when INT connected to GND
    pub int_pin_disabled: bool,
    /// Interrupt mode selection
    pub interrupt_mode: InterruptMode,
    /// INT interrupt latched or pulsed
    pub int_pin_mode: INTPinMode,
    /// Enable interrupt response on a predefined threshold cross.
    pub threshold_interrupt_enabled: bool,
    /// Enable interrupt response on conversion complete.
    pub conversion_complete_interrupt_enabled: bool,
}

#[derive(Default, Debug)]
pub struct DeviceConfig {
    /// I2C read mode
    pub i2c_read_mode: I2cReadMode,
    /// Additional sampling of the sensor data
    pub conv_avg: ConversionAverage,
    /// Temperature coefficient of the magnet
    pub mag_tempo: MagnetTemperatureCoefficient,
    /// Enables I2C CRC byte to be sent
    pub i2c_crc_enabled: bool,
    /// Selects Operating Mode and updates value based on operating
    /// mode if device transitions from Wake-up and sleep mode to Standby
    /// mode.
    pub operating_mode: OperatingMode,
    /// Selects a condition which initiates a single conversion based
    /// off already configured registers. A running conversion completes
    /// before executing a trigger. Redundant triggers are ignored.
    /// TRIGGER_MODE is available only during the mode explicitly
    /// mentioned in OPERATING_MODE.
    pub trigger_mode: TriggerMode,
    /// Selects whether the I2C glitch filter is enabled or disabled.
    pub i2c_glitch_filter_enabled: bool,
    /// Selects between Low Active Current Mode and Low Noise Mode.
    pub power_mode: LowPowerLowNoise,
    /// Select thresholds for the interrupt function
    pub threshold: Threshold,
}

#[derive(Default, Debug)]
pub struct SensorConfig {
    /// Selects the time spent in low power mode between conversions when operating_mode = ContinuousMeasure
    pub sleep_time: SleepTime,
    /// Enables data acquisition of the magnetic axis channel(s)
    pub mag_channel: MagneticChannel,
    /// Select the Z axis magnetic range from 2 different options
    pub z_range: Range,
    /// Select the XY axis magnetic range from 2 different options
    pub xy_range: Range,
    /// Enables angle calculation, magnetic gain, and offset corrections between two selected magnetic channels
    pub angle: Angle,
    /// Selects the axis for magnitude gain correction value entered in MAG_GAIN_CONFIG register
    pub gain_channel: MagGainChannel,
    /// Enables data acquisition of the temperature channel
    pub temperature_channel_enabled: bool,
    /// Selects the direction of threshold check. This bit is ignored when THR_HYST > 001b
    pub threshold_direction: MagThresholdDirection,
    /// Number of threshold crossings before the interrupt is asserted
    pub threshold_crossing_count: ThresholdCrossingCount,
}

impl<I2C> TMag5273<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Set the interrupt configuration.
    pub fn set_interrupts(&mut self, config: InterruptConfig) -> Result<(), TMag5273Error> {
        let register = InterruptConfigRegister::builder()
            .with_int_pin_disabled(config.int_pin_disabled)
            .with_interrupt_mode(config.interrupt_mode)
            .with_int_pin_mode(config.int_pin_mode)
            .with_threshold_interrupt_enabled(config.threshold_interrupt_enabled)
            .with_conversion_complete_interrupt_enabled(
                config.conversion_complete_interrupt_enabled,
            )
            .build();
        self.set_config_register(register)?;
        Ok(())
    }
    /// Set the device configuration.
    pub fn set_device_config(&mut self, config: DeviceConfig) -> Result<(), TMag5273Error> {
        let register = DeviceConfig1Register::builder()
            .with_i2c_read_mode(config.i2c_read_mode)
            .with_conv_avg(config.conv_avg)
            .with_mag_tempo(config.mag_tempo)
            .with_i2c_crc_enabled(config.i2c_crc_enabled)
            .build();
        self.set_config_register(register)?;
        let register = DeviceConfig2Register::builder()
            .with_operating_mode(config.operating_mode)
            .with_trigger_mode(config.trigger_mode)
            .with_i2c_glitch_filter_enabled(config.i2c_glitch_filter_enabled)
            .with_power_mode(config.power_mode)
            .with_threshold(config.threshold)
            .build();
        self.set_config_register(register)?;
        Ok(())
    }
    /// Set the sensor configuration.
    pub fn set_sensor_config(&mut self, config: SensorConfig) -> Result<(), TMag5273Error> {
        let register = SensorConfig1Register::builder()
            .with_sleep_time(config.sleep_time)
            .with_mag_channel(config.mag_channel)
            .build();
        self.set_config_register(register)?;
        let register = SensorConfig2Register::builder()
            .with_z_range(config.z_range)
            .with_xy_range(config.xy_range)
            .with_angle(config.angle)
            .with_gain_channel(config.gain_channel)
            .with_threshold_direction(config.threshold_direction)
            .with_threshold_crossing_count(config.threshold_crossing_count)
            .build();
        self.set_config_register(register)?;
        Ok(())
    }
    /// Get the sensor configuration.
    pub fn get_sensor_config(&mut self) -> Result<SensorConfig, TMag5273Error> {
        let config = self.get_dual_config_register::<SensorConfigRegisters>()?;
        let t_config = self.get_config_register::<TConfigRegister>()?;
        let (config1, config2) = (config.sensor_config1(), config.sensor_config2());
        let Ok(sleep_time) = config1.sleep_time() else {
            return Err(TMag5273Error::MalformedRegister);
        };
        Ok(SensorConfig {
            sleep_time,
            mag_channel: config1.mag_channel(),
            z_range: config2.z_range(),
            xy_range: config2.xy_range(),
            angle: config2.angle(),
            gain_channel: config2.gain_channel(),
            threshold_direction: config2.threshold_direction(),
            threshold_crossing_count: config2.threshold_crossing_count(),
            temperature_channel_enabled: t_config.temperature_channel_enabled(),
        })
    }
    /// Get the device configuration.
    pub fn get_device_config(&mut self) -> Result<DeviceConfig, TMag5273Error> {
        let config = self.get_dual_config_register::<DeviceConfigRegisters>()?;
        let (config1, config2) = (config.device_config1(), config.device_config2());
        let Ok(conv_avg) = config1.conv_avg() else {
            return Err(TMag5273Error::MalformedRegister);
        };
        let Ok(threshold) = config2.threshold() else {
            return Err(TMag5273Error::MalformedRegister);
        };
        Ok(DeviceConfig {
            i2c_read_mode: config1.i2c_read_mode(),
            conv_avg,
            mag_tempo: config1.mag_tempo(),
            i2c_crc_enabled: config1.i2c_crc_enabled(),
            operating_mode: config2.operating_mode(),
            trigger_mode: config2.trigger_mode(),
            i2c_glitch_filter_enabled: config2.i2c_glitch_filter_enabled(),
            power_mode: config2.power_mode(),
            threshold,
        })
    }
    /// Get the interrupt configuration.
    pub fn get_interrupt_config(&mut self) -> Result<InterruptConfig, TMag5273Error> {
        let config = self.get_config_register::<InterruptConfigRegister>()?;
        let Ok(interrupt_mode) = config.interrupt_mode() else {
            return Err(TMag5273Error::MalformedRegister);
        };
        Ok(InterruptConfig {
            int_pin_disabled: config.int_pin_disabled(),
            interrupt_mode,
            int_pin_mode: config.int_pin_mode(),
            threshold_interrupt_enabled: config.threshold_interrupt_enabled(),
            conversion_complete_interrupt_enabled: config.conversion_complete_interrupt_enabled(),
        })
    }
}
