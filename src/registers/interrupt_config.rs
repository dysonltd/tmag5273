use super::register_map::TMAG5273Register;
use crate::impl_register;
use arbitrary_int::u1;
use bitbybit::{bitenum, bitfield};

/// Interrupt mode select
/// This bit maps to INT_MODE in the datasheet
#[derive(Default, Debug)]
#[bitenum(u3, exhaustive = false)]
pub enum InterruptMode {
    /// No interrupt
    #[default]
    Off = 0,
    /// Interrupt through INT
    INTInterrupt = 1,
    /// Interrupt through INT except when I2C is busy.
    INTInterruptNotI2cBusy = 2,
    /// Interrupt through SCL Clock
    SCLInterrupt = 3,
    /// Interrupt through SCL except when I2C is busy.
    SCLInterruptNotI2cBusy = 4,
    // Reserved Bits 5 to 7
}

/// INT interrupt latched or pulsed
/// This bit maps to INT_STATE in the datasheet
#[derive(Default, Debug)]
#[bitenum(u1, exhaustive = true)]
pub enum INTPinMode {
    /// INT interrupt latched until clear by a primary addressing the device
    #[default]
    Latched = 0,
    /// INT interrupt pulse for 10us
    Pulsed = 1,
}

/// Represents the Interrupt Configuration Register.
#[bitfield(u8, default = 0)]
#[derive(Debug, PartialEq)]
pub struct InterruptConfigRegister {
    #[bit(0, rw)]
    /// Mask INT pin when INT connected to GND
    /// This bit maps to MASK_INTB in the datasheet
    pub int_pin_disabled: bool,
    #[bit(1, r)]
    /// 1 bit - Bit 1
    reserved: u1,
    #[bits(2..=4, rw)]
    /// Interrupt mode select
    /// This bit maps to INT_MODE in the datasheet
    pub interrupt_mode: Option<InterruptMode>,
    #[bit(5, rw)]
    /// INT interrupt latched or pulsed
    /// This bit maps to INT_STATE in the datasheet
    pub int_pin_mode: INTPinMode,
    #[bit(6, rw)]
    /// Enable interrupt response on a predefined threshold cross.
    /// This bit maps to THRSLD_INT in the datasheet
    pub threshold_interrupt_enabled: bool,
    #[bit(7, rw)]
    /// Enable interrupt response on conversion complete.
    /// This bit maps to RSLT_INT in the datasheet
    pub conversion_complete_interrupt_enabled: bool,
}

impl_register!(InterruptConfigRegister, TMAG5273Register::IntConfig1);
