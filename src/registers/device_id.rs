use super::register_map::TMAG5273Register;
use crate::impl_register;
use arbitrary_int::u6;
use bitbybit::{bitenum, bitfield};

/// Device version indicator. Reset value of DEVICE_ID depends on the orderable part number.
#[bitenum(u2)]
#[derive(Debug, PartialEq)]
pub enum DeviceId {
    /// ±40-mT and ±80-mT range
    TMAG5273X1 = 1,
    /// ±133-mT and ±266-mT range
    TMAG5273X2 = 2,
}

/// Represents the Device ID Register.
#[bitfield(u8, default = 0x01)]
pub struct DeviceIdRegister {
    /// 2 bits - Bit 0 and 1
    #[bits(0..=1, r)]
    pub device_id: Option<DeviceId>,
    /// 6 bits - Bit 2 to 7
    #[bits(2..=7, r)]
    reserved: u6,
}

impl_register!(DeviceIdRegister, TMAG5273Register::DeviceID);
