use super::register_map::TMAG5273Register;
use crate::impl_register;
use arbitrary_int::u7;
use bitbybit::bitfield;

/// Represents the Temperature Threshold Configuration Register.
#[bitfield(u8, default = 0)]
#[derive(Debug, PartialEq)]
pub struct TConfigRegister {
    /// 1 bit - Bit 0
    #[bit(0, rw)]
    /// Enables data acquisition of the temperature channel
    pub temperature_channel_enabled: bool,
    /// Temperature threshold code entered by user. The valid temperature threshold ranges are -41C to 170C with the threshold codes for -41C = 1Ah, and 170C = 34h. Resolution is 8 degree C/ LSB. Default 0h means no threshold comparison.
    #[bits(1..=7, rw)]
    pub t_thr_config: u7,
}
impl_register!(TConfigRegister, TMAG5273Register::TConfig);
