use super::register_map::TMAG5273Register;
use crate::impl_register;
use bitbybit::{bitenum, bitfield};

/// Selects Operating Mode and updates value based on operating
/// mode if device transitions from Wake-up and sleep mode to Standby
/// mode.
#[derive(Debug, Default)]
#[bitenum(u2, exhaustive = true)]
pub enum OperatingMode {
    /// Stand-by mode (starts new conversion at trigger event)
    #[default]
    StandBy = 0,
    /// Sleep mode
    Sleep = 1,
    /// Continuous measurement mode
    ContinuousMeasure = 2,
    /// Wake-up and sleep mode
    WakeUpAndSleep = 3,
}
/// Selects a condition which initiates a single conversion based
/// off already configured registers. A running conversion completes
/// before executing a trigger. Redundant triggers are ignored.
/// TRIGGER_MODE is available only during the mode explicitly
/// mentioned in OPERATING_MODE.
#[derive(Debug, Default)]
#[bitenum(u1, exhaustive = true)]
pub enum TriggerMode {
    /// Conversion Start at I2C Command Bits, DEFAULT
    #[default]
    Default = 0,
    /// Conversion starts through trigger signal at INT pin
    Int = 1,
}

/// Selects between Low Active Current Mode and Low Noise Mode.
/// This maps to LP_LN in the datasheet.
#[derive(Debug, Default)]
#[bitenum(u1, exhaustive = true)]
pub enum LowPowerLowNoise {
    /// Low Active Current Mode
    #[default]
    LowActiveCurrentMode = 0,
    /// Low Noise Mode
    LowNoiseMode = 1,
}

/// Select thresholds for the interrupt function
/// This maps to THR_HYST in the datasheet.
#[derive(Debug, Default)]
#[bitenum(u3, exhaustive = false)]
pub enum Threshold {
    /// Takes the 2's complement value of each x_THR_CONFIG register to create a magnetic threshold of the corresponding axis
    #[default]
    TwosComplement = 0,
    /// Takes the 7 LSB bits of the x_THR_CONFIG register to create two opposite magnetic thresholds (one north, and another south) of equal magnitude
    SevenLsb = 1,
    // Reserved
}

/// Represents the Device Configuration Register 2.
#[bitfield(u8, default = 0)]
#[derive(Debug, PartialEq)]
pub struct DeviceConfig2Register {
    #[bits(0..=1, rw)]
    /// Selects Operating Mode and updates value based on operating
    /// mode if device transitions from Wake-up and sleep mode to Standby
    /// mode.
    pub operating_mode: OperatingMode, // 2 bits - Bit 0 and 1
    #[bit(2, rw)]
    /// Selects a condition which initiates a single conversion based
    /// off already configured registers. A running conversion completes
    /// before executing a trigger. Redundant triggers are ignored.
    /// TRIGGER_MODE is available only during the mode explicitly
    /// mentioned in OPERATING_MODE.
    pub trigger_mode: TriggerMode, // 1 bit - Bit 2
    #[bit(3, rw)]
    /// Selects whether the I2C glitch filter is enabled or disabled.
    pub i2c_glitch_filter_enabled: bool, // 1 bit - Bit 3
    #[bit(4, rw)]
    /// Selects between Low Active Current Mode and Low Noise Mode.
    pub power_mode: LowPowerLowNoise, // 1 bit - Bit 4
    #[bits(5..=7, rw)]
    /// Select thresholds for the interrupt function
    pub threshold: Option<Threshold>, // 3 bits - Bit 5, 6 and 7
}
impl_register!(DeviceConfig2Register, TMAG5273Register::DeviceConfig2);
