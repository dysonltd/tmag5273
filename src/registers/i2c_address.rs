use super::register_map::TMAG5273Register;
use crate::impl_register;
use arbitrary_int::u7;
use bitbybit::bitfield;

/// Represents the I2C Address Register.
/// *PLEASE NOTE:* The reset value for this register is listed as 0x6A (106) in the datasheet,
/// however real-world testing indicates that the reset value for this register is 0x44 (68).
#[bitfield(u8)]
#[derive(Debug, PartialEq)]
pub struct I2cAddressRegister {
    // 1 bit - Bit 0
    #[bit(0, rw)]
    pub i2c_address_update_enabled: bool,
    // 7 bits - Bit 1 to 7
    // 7-bit default factory I2C address is loaded from OTP during first power up.
    // Change these bits to a new setting if a new I2C address is required (at each power cycle these bits must be written again to avoid going back to default factory address)
    #[bits(1..=7, rw)]
    pub i2c_address: u7,
}

impl_register!(I2cAddressRegister, TMAG5273Register::I2CAddress);
