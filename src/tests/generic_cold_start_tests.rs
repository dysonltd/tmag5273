//! This test suite is for the SparkFun breakout board for the TMAG5273 sensor on Mac OS.
//! It tests from a cold boot in which the configuration registers should be at their defaults

use crate::*;
const SENSOR_PART: DeviceVersion = DeviceVersion::TMAG5273B1;

macro_rules! test_default_register {
    ($register:ty, $sensor: ident) => {
        let read_register: $register = $sensor
            .get_config_register()
            .expect("Failed to get device status");
        assert_eq!(read_register, <$register>::default());
    };
}

pub fn generic_test_device_id<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    let device_id = mag_sensor.get_device_id().expect("Failed to get device id");
    assert_eq!(device_id, DeviceId::TMAG5273X1); // Ensure DeviceId derives PartialEq
}

pub fn generic_test_manufacturer_id<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    let manufacturer_id = mag_sensor
        .get_manufacturer_id()
        .expect("Failed to get manufacturer id");

    assert_eq!(manufacturer_id, MANUFACTURER_ID_VALUE);
}

pub fn generic_test_registers<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    test_default_register!(ConversionStatusRegister, mag_sensor);
    test_default_register!(DeviceConfig1Register, mag_sensor);
    test_default_register!(DeviceConfig2Register, mag_sensor);
    test_default_register!(DeviceStatusRegister, mag_sensor);
    test_default_register!(InterruptConfigRegister, mag_sensor);
    test_default_register!(SensorConfig1Register, mag_sensor);
    test_default_register!(SensorConfig2Register, mag_sensor);
    test_default_register!(TConfigRegister, mag_sensor);
}

pub fn generic_test_default_i2c_address<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    let i2c_register: I2cAddressRegister = mag_sensor
        .get_config_register()
        .expect("Failed to read I2C address register");

    let i2c_address = i2c_register.raw_value();

    // I2C address register read returns the 8-bit address of the device
    // Therefore we must shift right to obtain the 7-bit address
    assert_eq!(i2c_address >> 1, SENSOR_PART.get_default_address());
}

pub fn generic_test_get_magnitude_first_boot<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    let magnitude = mag_sensor.get_magnitude().expect("Failed to get magnitude");

    assert_eq!(magnitude, 0); // This will be 0.0 on first boot! Checked on bus itself
}

pub fn generic_test_get_xyz_thresholds_first_boot<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    let x_threshold = mag_sensor
        .get_mag_threshold(Axis::X)
        .expect("Failed to get x threshold");

    assert_eq!(x_threshold, 0.0); // This will be 0 on first boot! Checked on bus itself

    let y_threshold = mag_sensor
        .get_mag_threshold(Axis::Y)
        .expect("Failed to get y threshold");
    assert_eq!(y_threshold, 0.0); // This will be 0 on first boot! Checked on bus itself

    let z_threshold = mag_sensor
        .get_mag_threshold(Axis::Z)
        .expect("Failed to get x threshold");
    assert_eq!(z_threshold, 0.0); // This will be 0 on first boot! Checked on bus itself
}

pub fn generic_test_magnetic_gain<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    let gain = mag_sensor
        .get_magnetic_gain()
        .expect("Failed to get magnetic gain");
    assert_eq!(gain, 0);
}

pub fn generic_test_magnetic_offset_invalid_at_boot<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    assert_eq!(
        Err(TMag5273Error::WrongMode),
        mag_sensor.get_magnetic_offset(MagneticChannelOffset::FirstAxis),
        "You should not be able to get magnetic offset on first boot!"
    );
    assert_eq!(
        Err(TMag5273Error::WrongMode),
        mag_sensor.get_magnetic_offset(MagneticChannelOffset::SecondAxis),
        "You should not be able to get magnetic offset on first boot!"
    );
}

pub fn generic_test_temperature_invalid_at_boot<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    assert_eq!(
        Err(TMag5273Error::ChannelDisabled),
        mag_sensor.get_temperature(),
        "You should not be able to get temperature on first boot!"
    );
}

/// Helper function to test the get_axis_data method
fn generic_get_axis_data<I2C>(i2c: &mut I2C, axis: Axis)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    assert_eq!(
        Err(TMag5273Error::WrongMode),
        mag_sensor.get_mag_data(axis),
        "You should not be able to get axis data on first boot!"
    );
}

pub fn generic_test_get_data_methods<I2C>(mut i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    generic_get_axis_data(&mut i2c, Axis::X);
    generic_get_axis_data(&mut i2c, Axis::Y);
    generic_get_axis_data(&mut i2c, Axis::Z);
}

pub fn generic_test_get_angle<I2C>(i2c: I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut mag_sensor = TMag5273::new(i2c, SENSOR_PART).unwrap();
    assert_eq!(
        Err(TMag5273Error::ChannelDisabled),
        mag_sensor.get_angle(),
        "You should not be able to get angle on first boot!"
    );
}
