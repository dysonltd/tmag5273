use super::register_map::TMAG5273Register;
use crate::impl_register;
use arbitrary_int::{u2, u3};
use bitbybit::bitfield;

/// Represents the Conversion Status Register.
#[bitfield(u8, default = 0x10)]
#[derive(Debug, PartialEq)]
pub struct ConversionStatusRegister {
    /// Conversion data buffer is ready to be read.
    #[bit(0, r)]
    pub conversion_ready: bool, // 1 bit - Bit 0
    /// Detect any internal diagnostics fail which include VCC UV, internal
    /// memory CRC error, INT pin error and internal clock error. Ignore this
    /// bit status if VCC < 2.3V
    #[bit(1, r)]
    pub diagnostic_error: bool, // 1 bit - Bit 1
    #[bits(2..=3, r)]
    reserved: u2, // 2 bits - Bit 2 to 3
    /// Device powered up, or experienced power-on-reset (POR). Bit is clear when
    /// host writes back '1'. (0 - No POR 1 - POR oc)
    #[bit(4, r)]
    pub power_on_reset: bool, // 1 bit - Bit 4
    /// Rolling Count of Conversion Data Sets
    #[bits(5..=7, r)]
    pub set_count: u3, // 3 bits - Bit 5 to 7
}

impl_register!(ConversionStatusRegister, TMAG5273Register::ConvStatus);
