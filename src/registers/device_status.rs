use super::register_map::TMAG5273Register;
use crate::impl_register;
use arbitrary_int::u3;
use bitbybit::{bitenum, bitfield};

/// Indicates the level that the device is reading back from INT pin. The
/// reset value of DEVICE_STATUS depends on the status of the INT pin at power-up
/// This maps to the INTB_RB bit in the Device Status Register.
#[bitenum(u1, exhaustive = true)]
pub enum IntPinReadBack {
    /// INT pin drive low
    Low = 0,
    /// INT pin status high
    High = 1,
}

/// Represents the Device Status Register.
#[bitfield(u8, default = 0x10)]
#[derive(Debug, PartialEq)]
pub struct DeviceStatusRegister {
    #[bit(0, r)]
    /// VCC Under Voltage error (0 - No error, 1 - Error)
    pub vcc_under_voltage_error: bool, // 1 bit - Bit 0
    #[bit(1, r)]
    /// One Time Programmable (OTP) CRC error (0 - No error, 1 - Error)
    pub crc_error: bool, // 1 bit - Bit 1
    #[bit(2, r)]
    /// INT pin error (0 - No error, 1 - Error)
    pub int_pin_error: bool, // 1 bit - Bit 2
    #[bit(3, r)]
    /// Oscillator error (0 - No error, 1 - Error)
    pub oscillator_error: bool, // 1 bit - Bit 3
    #[bit(4, r)]
    /// INT pin Read Back
    pub int_pin_read_back: IntPinReadBack, // 1 bit - Bit 4
    #[bits(5..=7, r)]
    reserved: u3, // 3 bits - Bit 5 to 7
}

impl_register!(DeviceStatusRegister, TMAG5273Register::DeviceStatus);
