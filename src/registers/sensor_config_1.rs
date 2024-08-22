use super::register_map::TMAG5273Register;
use crate::impl_register;
use bitbybit::{bitenum, bitfield};

/// Selects the time spent in low power mode between conversions when OperatingMode = ContinuousMeasure
#[derive(Default, Debug, PartialEq)]
#[bitenum(u4, exhaustive = false)]
pub enum SleepTime {
    // 1 ms
    #[default]
    Ms1 = 0x0,
    // 5 ms
    Ms5 = 0x1,
    // 10 ms
    Ms10 = 0x2,
    // 15 ms
    Ms15 = 0x3,
    // 20 ms
    Ms20 = 0x4,
    // 30 ms
    Ms30 = 0x5,
    // 50 ms
    Ms50 = 0x6,
    // 100 ms
    Ms100 = 0x7,
    // 500 ms
    Ms500 = 0x8,
    // 1000 ms
    Ms1000 = 0x9,
    // 2000 ms
    Ms2000 = 0xA,
    // 5000 ms
    Ms5000 = 0xB,
    // 10000 ms
    Ms10000 = 0xC,
}
/// Enables data acquisition of the magnetic axis channel(s)
/// This maps to MAG_CH_EN in the datasheet.
#[derive(Debug, PartialEq, Default)]
#[bitenum(u4, exhaustive = true)]
pub enum MagneticChannel {
    // All magnetic channels are disabled
    #[default]
    Default = 0x0,
    // X Channel is enabled
    X = 0x1,
    // Y Channel is enabled
    Y = 0x2,
    // X and Y Channels are enabled
    XY = 0x3,
    // Z Channel is enabled
    Z = 0x4,
    // X and Z Channels are enabled
    XZ = 0x5,
    // Y and Z Channels are enabled
    YZ = 0x6,
    // All magnetic channels are enabled
    XYZ = 0x7,
    // X and Y Channels are enabled, X Channel is repeated
    XYX = 0x8,
    // Y and X Channels are enabled, Y Channel is repeated
    YXY = 0x9,
    // Y and Z Channels are enabled, Y Channel is repeated
    YZY = 0xA,
    // X and Z Channels are enabled, X Channel is repeated
    XZX = 0xB,
    // The rest are reserved
    Reserved1 = 0xC,
    Reserved2 = 0xD,
    Reserved3 = 0xE,
    Reserved4 = 0xF,
}

/// Represents the Sensor Configuration Register 1.
#[bitfield(u8, default = 0)]
#[derive(Debug, PartialEq)]
pub struct SensorConfig1Register {
    #[bits(0..=3, rw)]
    pub sleep_time: Option<SleepTime>, // 4 bits - Bit 0 to 3
    #[bits(4..=7, rw)]
    pub mag_channel: MagneticChannel, // 4 bits - Bit 4 to 7
}
impl_register!(SensorConfig1Register, TMAG5273Register::SensorConfig1);
