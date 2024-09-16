use embedded_hal::i2c::SevenBitAddress;

use crate::{registers::TMAG5273Register, DeviceId};

/// Device version
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum DeviceVersion {
    #[default]
    TMAG5273A1,
    TMAG5273B1,
    TMAG5273C1,
    TMAG5273D1,
    TMAG5273A2,
    TMAG5273B2,
    TMAG5273C2,
    TMAG5273D2,
}

impl DeviceVersion {
    /// Gets the default i2c address associated with the hardware version
    pub fn get_default_address(self) -> SevenBitAddress {
        match self {
            Self::TMAG5273A1 | Self::TMAG5273A2 => 0x35,
            Self::TMAG5273B1 | Self::TMAG5273B2 => 0x22,
            Self::TMAG5273C1 | Self::TMAG5273C2 => 0x78,
            Self::TMAG5273D1 | Self::TMAG5273D2 => 0x44,
        }
    }
    
    /// Gets the device id associated with the hardware version
    pub fn get_device_id(self) -> DeviceId {
        match self {
            Self::TMAG5273A1 | Self::TMAG5273B1 | Self::TMAG5273C1 | Self::TMAG5273D1 => {
                DeviceId::TMAG5273X1
            }
            Self::TMAG5273A2 | Self::TMAG5273B2 | Self::TMAG5273C2 | Self::TMAG5273D2 => {
                DeviceId::TMAG5273X2
            }
        }
    }
}
// TMAG5273 All Sensor Channel Data
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TMag5273ChannelData {
    /// X Axis in mT
    pub x: f32,
    /// Y Axis in mT
    pub y: f32,
    /// Z Axis  in mT
    pub z: f32,
    /// Temperature in degrees C
    pub temp: f32,
}

/// Magnetic field Axis
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Magnetic Offsets
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MagneticChannelOffset {
    /// 8-bit, 2's complement offset value determined by a primary to adjust
    /// first axis offset value. The range of possible offset valid entrees can
    /// be +/-128. The offset value is calculated by multiplying bit resolution
    /// with the entered value.
    FirstAxis,
    /// 8-bit, 2's complement offset value determined by a primary to adjust
    /// second axis offset value. The range of possible offset valid entrees
    /// can be +/-128. The offset value is calculated by multiplying bit
    /// resolution with the entered value.
    SecondAxis,
}

impl From<MagneticChannelOffset> for TMAG5273Register {
    fn from(offset: MagneticChannelOffset) -> Self {
        match offset {
            MagneticChannelOffset::FirstAxis => TMAG5273Register::MagOffsetConfig1,
            MagneticChannelOffset::SecondAxis => TMAG5273Register::MagOffsetConfig2,
        }
    }
}

/// Errors for the TMAG5273 device
#[derive(PartialEq, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TMag5273Error {
    /// No device at the devices address was found on the bus
    NotConnected,
    /// The device at the address is not a TMAG5273
    WrongDevice,
    /// The data returned from the device was malformed and could not be converted properly
    MalformedRegister,
    /// The device is in the wrong mode for the operation
    WrongMode,
    /// Input is out of range
    OutOfRange,
    /// Channel is not enabled
    ChannelDisabled,
    /// An I2C error occurred
    I2c(embedded_hal::i2c::ErrorKind),
}

impl<E: embedded_hal::i2c::Error> From<E> for TMag5273Error {
    fn from(e: E) -> Self {
        TMag5273Error::I2c(e.kind())
    }
}

// Allows Error Bubbling when working with both std and no-std rust
impl core::error::Error for TMag5273Error {}

impl core::fmt::Display for TMag5273Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TMag5273Error::NotConnected => write!(f, "No device at the device's address was found on the bus"),
            TMag5273Error::WrongDevice => write!(f, "The device at the address is not a TMAG5273"),
            TMag5273Error::MalformedRegister => write!(f, "The data returned from the device was malformed and could not be converted properly"),
            TMag5273Error::WrongMode => write!(f, "The device is in the wrong mode for the operation"),
            TMag5273Error::OutOfRange => write!(f, "Input is out of range"),
            TMag5273Error::ChannelDisabled => write!(f, "Channel is not enabled"),
            TMag5273Error::I2c(e) => write!(f, "An I2C error occurred: {:?}", e),
        }
    }
}
