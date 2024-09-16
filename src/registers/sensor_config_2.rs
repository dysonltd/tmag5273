use super::register_map::TMAG5273Register;
use crate::{impl_register, types::DeviceVersion};
use arbitrary_int::u1;
use bitbybit::{bitenum, bitfield};

/// Select the Z/XY axis magnetic range from 2 different options
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Default)]
pub enum Range {
    /// ±40mT (TMAG5273X1) or ±133mT (TMAG5273X2), DEFAULT
    #[default]
    Low = 0,
    /// ±80mT (TMAG5273X1) or ±266mT (TMAG5273X2)
    High = 1,
}

impl Range {
    /// Returns the range value in mT
    /// 40mT or 80mT for X1 and 133mT or 266mT for X2
    pub fn get_range(&self, version: DeviceVersion) -> f32 {
        match (self, version) {
            (
                Range::Low,
                DeviceVersion::TMAG5273A1
                | DeviceVersion::TMAG5273B1
                | DeviceVersion::TMAG5273C1
                | DeviceVersion::TMAG5273D1,
            ) => 40.0,
            (
                Range::Low,
                DeviceVersion::TMAG5273A2
                | DeviceVersion::TMAG5273B2
                | DeviceVersion::TMAG5273C2
                | DeviceVersion::TMAG5273D2,
            ) => 133.0,
            (
                Range::High,
                DeviceVersion::TMAG5273A1
                | DeviceVersion::TMAG5273B1
                | DeviceVersion::TMAG5273C1
                | DeviceVersion::TMAG5273D1,
            ) => 80.0,
            (
                Range::High,
                DeviceVersion::TMAG5273A2
                | DeviceVersion::TMAG5273B2
                | DeviceVersion::TMAG5273C2
                | DeviceVersion::TMAG5273D2,
            ) => 266.0,
        }
    }
}

/// Enables angle calculation, magnetic gain, and offset corrections between two selected magnetic channels
/// This matches the ANGLE_EN field in the datasheet
#[derive(Debug, PartialEq, Default)]
#[bitenum(u2, exhaustive = true)]
pub enum Angle {
    /// No angle calculation, magnitude gain, and offset correction enabled
    #[default]
    Disabled = 0,
    /// X 1st, Y 2nd
    XY = 1,
    /// Y 1st, Z 2nd
    YZ = 2,
    /// X 1st, Z 2nd
    XZ = 3,
}
/// Selects the axis for magnitude gain correction value entered in MAG_GAIN_CONFIG register
/// This matches the MAG_GAIN_CH field in the datasheet
#[derive(Debug, PartialEq, Default)]
#[bitenum(u1, exhaustive = true)]
pub enum MagGainChannel {
    /// 1st channel is selected for gain adjustment
    #[default]
    First = 0,
    /// 2nd channel is selected for gain adjustment
    Second = 1,
}

/// Selects the direction of threshold check. This bit is ignored when THR_HYST > 001b
/// This matches the MAG_THR_DIR field in the datasheet
#[derive(Debug, PartialEq, Default)]
#[bitenum(u1, exhaustive = true)]
pub enum MagThresholdDirection {
    /// Sets interrupt for field above the threshold
    #[default]
    Above = 0,
    /// Sets interrupt for field below the threshold
    Below = 1,
}

/// Number of threshold crossings before the interrupt is asserted
/// This matches the THR_CNT field in the datasheet
#[derive(Debug, PartialEq, Default)]
#[bitenum(u1, exhaustive = true)]
pub enum ThresholdCrossingCount {
    /// 1 threshold crossing
    #[default]
    One = 0,
    /// 4 threshold crossing
    Four = 1,
}

/// Represents the Sensor Configuration Register 2.
#[bitfield(u8, default = 0)]
#[derive(Debug, PartialEq)]
pub struct SensorConfig2Register {
    #[bit(0, rw)]
    /// 1 bit - Bit 0
    pub z_range: Range,
    #[bit(1, rw)]
    /// 1 bit - Bit 1
    pub xy_range: Range,
    #[bits(2..=3, rw)]
    /// 2 bits - Bit 2 and 3
    pub angle: Angle,
    #[bit(4, rw)]
    /// 1 bit - Bit 4
    pub gain_channel: MagGainChannel,
    #[bit(5, rw)]
    /// 1 bit - Bit 5
    pub threshold_direction: MagThresholdDirection,
    #[bit(6, rw)]
    /// 1 bit - Bit 6
    pub threshold_crossing_count: ThresholdCrossingCount,
    #[bit(7, r)]
    /// 1 bit - Bit 7
    reserved: u1,
}

impl_register!(SensorConfig2Register, TMAG5273Register::SensorConfig2);
