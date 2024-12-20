use core::assert_eq;
use core::cell::RefCell;
use embedded_hal_bus::i2c::RefCellDevice;
use registers::*;
use tmag5273::*;
use types::*;

const SENSOR_PART: DeviceVersion = DeviceVersion::TMAG5273B1;

/// Helper macro to clear a register to its default value
macro_rules! reset_register {
    ($register:ty, $sensor:ident) => {
        $sensor
            .set_config_register(<$register>::default())
            .expect("Failed to set register to default settings");
        assert_eq!(
            $sensor
                .get_config_register::<$register>()
                .expect("Failed to read register"),
            <$register>::default()
        )
    };
}

pub fn generic_test_is_connected<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    let is_connected = mag_sensor.is_connected();
    assert!(is_connected);
}

pub fn generic_test_create_tmag5273<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mag_sensor = TMag5273::new(i2c, SENSOR_PART);
    assert!(mag_sensor.is_ok());
}

pub fn generic_test_set_reset_device_config_1_register<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor =
        TMag5273::new(i2c, SENSOR_PART).expect("Failed to create mag sensor instance");
    reset_register!(DeviceConfig1Register, mag_sensor);

    let new_device_config_1 = DeviceConfig1Register::builder()
        .with_i2c_read_mode(I2cReadMode::Standard3Byte)
        .with_conv_avg(ConversionAverage::X4)
        .with_mag_tempo(MagnetTemperatureCoefficient::Zero2Compensation)
        .with_i2c_crc_enabled(false)
        .build();
    // Set the new Device Config 1 Register
    mag_sensor
        .set_config_register(new_device_config_1)
        .expect("Failed to set Device Config Register 1");

    let applied_device_config_1: DeviceConfig1Register = mag_sensor
        .get_config_register()
        .expect("Failed to get Device Config Register 1");

    assert_eq!(applied_device_config_1, new_device_config_1); // Check that it matches what we set

    // Reset the Device Config 1 Register
    reset_register!(DeviceConfig1Register, mag_sensor);
}

pub fn generic_test_reset_device_config_2_register<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor =
        TMag5273::new(i2c, SENSOR_PART).expect("Failed to create mag sensor instance");
    reset_register!(DeviceConfig2Register, mag_sensor);

    let new_device_config_2 = DeviceConfig2Register::builder()
        .with_operating_mode(OperatingMode::ContinuousMeasure)
        .with_trigger_mode(TriggerMode::Int)
        .with_i2c_glitch_filter_enabled(false)
        .with_power_mode(LowPowerLowNoise::LowNoiseMode)
        .with_threshold(Threshold::SevenLsb)
        .build();

    // Set the new Device Config 2 Register
    mag_sensor
        .set_config_register(new_device_config_2)
        .expect("Failed to set Device Config Register 2");

    let applied_device_config_2: DeviceConfig2Register = mag_sensor
        .get_config_register()
        .expect("Failed to get Device Config Register 2");

    assert_eq!(applied_device_config_2, new_device_config_2); // Check that it matches what we set

    // Reset the Device Config 2 Register
    reset_register!(DeviceConfig2Register, mag_sensor);
}

pub fn generic_test_set_reset_i2c_address_register<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let i2c_ref_cell = RefCell::new(i2c);
    let new_address = 0x55;

    // Set the new I2C Address Register
    {
        // Scope to drop the mutable borrow of mag_sensor
        let mut mag_sensor = TMag5273::new(RefCellDevice::new(&i2c_ref_cell), SENSOR_PART).unwrap();

        let new_i2c_address_register = I2cAddressRegister::builder()
            .with_i2c_address_update_enabled(true)
            .with_i2c_address(arbitrary_int::u7::new(new_address))
            .build();

        mag_sensor
            .set_config_register(new_i2c_address_register)
            .expect("Failed to set I2C Address Register");
    }

    // Connect to the sensor on the new address
    {
        // Scope to drop the mutable borrow of mag_sensor
        let mut mag_sensor =
            TMag5273::new_with_address(RefCellDevice::new(&i2c_ref_cell), new_address, SENSOR_PART)
                .unwrap();

        // Check that the Sensor is connected with the new address
        assert!(mag_sensor.is_connected());

        // Reset the I2C Address Register
        let reset_i2c_register = I2cAddressRegister::builder()
            .with_i2c_address_update_enabled(false)
            .with_i2c_address(arbitrary_int::u7::new(new_address))
            .build();
        mag_sensor
            .set_config_register(reset_i2c_register)
            .expect("Failed to reset I2C Address Register");

        assert!(!mag_sensor.is_connected());
    }
    // Check if the device is back to normal address now

    let mut mag_sensor = TMag5273::new(RefCellDevice::new(&i2c_ref_cell), SENSOR_PART).unwrap();
    assert!(mag_sensor.is_connected());
}

pub fn generic_test_set_reset_int_config_1_register<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    reset_register!(InterruptConfigRegister, mag_sensor);

    let new_int_config_1 = InterruptConfigRegister::builder()
        .with_int_pin_disabled(true)
        .with_interrupt_mode(InterruptMode::INTInterrupt)
        .with_int_pin_mode(INTPinMode::Pulsed)
        .with_threshold_interrupt_enabled(true)
        .with_conversion_complete_interrupt_enabled(true)
        .build();

    // Set the new Int Config 1 Register
    mag_sensor
        .set_config_register(new_int_config_1)
        .expect("Failed to set Int Config Register 1");

    let applied_int_config_1: InterruptConfigRegister = mag_sensor
        .get_config_register()
        .expect("Failed to get Int Config Register 1");

    assert_eq!(applied_int_config_1, new_int_config_1); // Check that it matches what we set

    // Reset the Int Config 1 Register
    reset_register!(InterruptConfigRegister, mag_sensor);
}

pub fn generic_test_set_reset_sensor_config_1_register<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor =
        TMag5273::new(i2c, SENSOR_PART).expect("Failed to create mag sensor instance");
    reset_register!(SensorConfig1Register, mag_sensor);

    // Set the new Sensor Config 1 Register
    let new_sensor_config_1 = SensorConfig1Register::builder()
        .with_sleep_time(SleepTime::Ms1000)
        .with_mag_channel(MagneticChannel::XZX)
        .build();

    mag_sensor
        .set_config_register(new_sensor_config_1)
        .expect("Failed to set Sensor Config Register 1");

    let applied_sensor_config_1: SensorConfig1Register = mag_sensor
        .get_config_register()
        .expect("Failed to get Sensor Config Register 1");

    assert_eq!(applied_sensor_config_1, new_sensor_config_1); // Check that it matches what we set

    // Reset the Sensor Config 1 Register
    mag_sensor
        .set_config_register(SensorConfig1Register::default()) // Set it to all zeros
        .expect("Failed to reset Sensor Config Register 1");

    reset_register!(SensorConfig1Register, mag_sensor);
}

pub fn generic_test_set_reset_sensor_config_2_register<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor =
        TMag5273::new(i2c, SENSOR_PART).expect("Failed to create mag sensor instance");
    reset_register!(SensorConfig2Register, mag_sensor);

    let new_sensor_config_2 = SensorConfig2Register::builder()
        .with_z_range(Range::High)
        .with_xy_range(Range::High)
        .with_angle(Angle::XY)
        .with_gain_channel(MagGainChannel::Second)
        .with_threshold_direction(MagThresholdDirection::Below)
        .with_threshold_crossing_count(ThresholdCrossingCount::Four)
        .build();

    mag_sensor
        .set_config_register(new_sensor_config_2)
        .expect("Failed to set Sensor Config Register 2");

    let applied_sensor_config_2: SensorConfig2Register = mag_sensor
        .get_config_register()
        .expect("Failed to get Sensor Config Register 2");

    assert_eq!(applied_sensor_config_2, new_sensor_config_2); // Check that it matches what we set
    reset_register!(SensorConfig2Register, mag_sensor);
}

pub fn generic_test_set_reset_t_config_register<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    // Put the device in a known state

    let mut mag_sensor =
        TMag5273::new(i2c, SENSOR_PART).expect("Failed to create mag sensor instance");
    reset_register!(TConfigRegister, mag_sensor);

    let temp_threshold = 54;
    let new_t_config = TConfigRegister::builder()
        .with_temperature_channel_enabled(true)
        .with_t_thr_config(arbitrary_int::u7::new(temp_threshold))
        .build();

    // Set the new T Config Register
    mag_sensor
        .set_config_register(new_t_config)
        .expect("Failed to set T Config Register");

    let applied_t_config: TConfigRegister = mag_sensor
        .get_config_register()
        .expect("Failed to get T Config Register");

    assert_eq!(applied_t_config, new_t_config); // Check that it matches what we set
    reset_register!(TConfigRegister, mag_sensor);
}

//TODO: Add Tests for setting Thresholds!
