use super::register_map::TMAG5273Register;
use crate::impl_register;
use bitbybit::{bitenum, bitfield};

/// Defines the I2C read mode
/// This maps to IR in the datasheet.
#[derive(Debug, Default)]
#[bitenum(u2, exhaustive = true)]
pub enum I2cReadMode {
    /// Standard I2C 3-byte read command
    #[default]
    Standard3Byte = 0,
    /// 1-byte I2C read command for 16 bit sensor data and conversion status
    OneByte16Bit = 1,
    /// 1-byte I2C read command for 8 bit sensor MSB data and conversion status
    TwoByte8Bit = 2,
    /// Reserved
    Reserved = 3,
}
/// Enables additional sampling of the sensor data to reduce the noise effect (or to increase resolution)
/// This maps to CONV_AVG in the datasheet.
#[derive(Debug, Default)]
#[bitenum(u3, exhaustive = false)]
pub enum ConversionAverage {
    /// 1x average, 10.0-kSPS (3-axes) or 20-kSPS (1 axis)
    #[default]
    X1 = 0,
    /// 2x average, 5.7-kSPS (3-axes) or 13.3-kSPS (1 axis)
    X2 = 1,
    /// 4x average, 3.1-kSPS (3-axes) or 8.0-kSPS (1 axis)
    X4 = 2,
    /// 8x average, 1.6-kSPS (3-axes) or 4.4-kSPS (1 axis)
    X8 = 3,
    /// 16x average, 0.8-kSPS (3-axes) or 2.4-kSPS (1 axis)
    X16 = 4,
    /// 32x average, 0.4-kSPS (3-axes) or 1.2-kSPS (1 axis)
    X32 = 5,
    // Reserved
}
/// Temperature coefficient of the magnet
/// This maps to MAG_TEMPO in the datasheet.
#[derive(Debug, Default)]
#[bitenum(u2, exhaustive = true)]
pub enum MagnetTemperatureCoefficient {
    /// 0% (No temperature compensation)
    #[default]
    ZeroCompensation = 0,
    /// 0.12%/ deg C (NdBFe)
    Zero12Compensation = 1,
    /// Reserved
    Reserved = 2,
    /// 0.2%/deg C (Ceramic)
    Zero2Compensation = 3,
}

/// Represents the Device Configuration Register 1.
#[bitfield(u8, default = 0)]
#[derive(Debug, PartialEq)]
pub struct DeviceConfig1Register {
    #[bits(0..=1, rw)]
    /// I2C read mode - 2 bits (Bit 0 and 1)
    pub i2c_read_mode: I2cReadMode,
    #[bits(2..=4, rw)]
    /// Additional sampling of the sensor data - 3 bits (Bit 2, 3, and 4)
    pub conv_avg: Option<ConversionAverage>,
    #[bits(5..=6, rw)]
    /// Temperature coefficient of the magnet - 2 bits (Bit 5 and 6)
    pub mag_tempo: MagnetTemperatureCoefficient,
    #[bit(7, rw)]
    /// Enables I2C CRC byte to be sent - 1 bit (Bit 7)
    pub i2c_crc_enabled: bool,
}

impl_register!(DeviceConfig1Register, TMAG5273Register::DeviceConfig1);
