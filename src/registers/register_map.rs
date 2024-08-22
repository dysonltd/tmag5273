use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, IntoPrimitive, TryFromPrimitive, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TMAG5273Register {
    /// Represents the Device Configuration Register 1.
    DeviceConfig1 = 0x00,
    /// Represents the Device Configuration Register 2.
    DeviceConfig2 = 0x01,
    /// Represents the Sensor Configuration Register 1.
    SensorConfig1 = 0x02,
    /// Represents the Sensor Configuration Register 2.
    SensorConfig2 = 0x03,
    ///8-bit, 2's complement X axis threshold code for limit
    ///check. The range of possible threshold entrees can be
    ///+/-128. The threshold value in mT is calculated for
    ///A1 as (40(1+X_Y_RANGE)/128)*X_THR_CONFIG, for A2 as
    ///(133(1+X_Y_RANGE)/128)*X_THR_CONFIG. Default 0h means no
    ///threshold comparison.
    XThrConfig = 0x04,
    ///8-bit, 2's complement Y axis threshold code for limit
    ///check. The range of possible threshold entrees can be
    ///+/-128. The threshold value in mT is calculated for
    ///A1 as (40(1+X_Y_RANGE)/128)*X_THR_CONFIG, for A2 as
    ///(133(1+X_Y_RANGE)/128)*X_THR_CONFIG. Default 0h means no
    ///threshold comparison.
    YThrConfig = 0x05,
    /// 8-bit, 2's complement X axis threshold code for limit
    /// check. The range of possible threshold entrees can be
    /// +/-128. The threshold value in mT is calculated for
    /// A1 as (40(1+X_Y_RANGE)/128)*X_THR_CONFIG, for A2 as
    /// (133(1+X_Y_RANGE)/128)*X_THR_CONFIG. Default 0h means no
    /// threshold comparison
    ZThrConfig = 0x06,
    /// Represents the Temperature Threshold Configuration Register.
    TConfig = 0x07,
    /// Represents the Interrupt Configuration Register.
    IntConfig1 = 0x08,
    /// 8-bit gain value determined by a primary to adjust a Hall axis
    /// gain. The particular axis is selected based off the settings of
    /// MAG_GAIN_CH and ANGLE_EN register bits. The binary 8-bit input
    /// is interpreted as a fractional value in between 0 and 1 based off
    /// the formula, 'user entered value in decimal/256'. Gain value of 0 is
    /// interpreted by the device as 1
    MagGainConfig = 0x09,
    /// 8-bit, 2's complement offset value determined by a primary to adjust
    /// first axis offset value. The range of possible offset valid entrees can
    /// be +/-128. The offset value is calculated by multiplying bit resolution
    /// with the entered value.
    MagOffsetConfig1 = 0x0A,
    /// 8-bit, 2's complement offset value determined by a primary to adjust
    /// second axis offset value. The range of possible offset valid entrees
    /// can be +/-128. The offset value is calculated by multiplying bit
    /// resolution with the entered value.
    MagOffsetConfig2 = 0x0B,
    /// Represents the I2C Address Register.
    I2CAddress = 0x0C,
    /// Represents the Device ID Register.
    DeviceID = 0x0D,
    /// 8-bit unique manufacturer ID
    ManufacturerIdLsb = 0x0E,
    /// 8-bit unique manufacturer ID
    ManufacturerIdMsb = 0x0F,
    /// T-channel data conversion results, MSB 8 bits.
    TMsbResult = 0x10,
    /// T-channel data conversion results, LSB 8 bits
    TLsbResult = 0x11,
    /// X-channel data conversion results, MSB 8 bits
    XMsbResult = 0x12,
    /// X-channel data conversion results, LSB 8 bits
    XLsbResult = 0x13,
    /// Y-channel data conversion results, MSB 8 bits
    YMSBResult = 0x14,
    /// Y-channel data conversion results, LSB 8 bits.
    YLsbResult = 0x15,
    /// Z-channel data conversion results, MSB 8 bits.
    ZMSBResult = 0x16,
    /// Z-channel data conversion results, LSB 8 bits
    ZLsbResult = 0x17,
    /// Represents the Conversion Status Register.
    ConvStatus = 0x18,
    /// Angle measurement result in degree. The data is displayed
    /// from 0 to 360 degree in 13 LSB bits after combining the
    /// ANGLE_RESULT_MSB and _LSB bits. The 4 LSB bits allocated for
    /// fraction of an angle in the format (xxxx/16).
    AngleResultMSB = 0x19,
    /// Angle measurement result in degree. The data is displayed
    /// from 0 to 360 degree in 13 LSB bits after combining the
    /// ANGLE_RESULT_MSB and _LSB bits. The 4 LSB bits allocated for
    /// fraction of an angle in the format (xxxx/16).
    AngleResultLSB = 0x1A,
    /// Resultant vector magnitude (during angle measurement) result. This
    /// value should be constant during 360 degree measurements
    MagnitudeResult = 0x1B,
    /// Represents the Device Status Register.
    DeviceStatus = 0x1C,
}

/// Wrapper Trait around the BitField Registers to allow the user to use Generic Functions rather than
/// having to write a function for each register.
pub trait BitFieldDeviceConfiguration {
    fn raw_value(&self) -> u8;
    fn new_with_raw_value(raw_value: u8) -> Self;
    fn get_address() -> TMAG5273Register;
}

/// Wrapper Trait
pub trait ByteFieldDeviceConfiguration {
    fn raw_value(&self) -> u16;
    fn new_with_raw_value(raw_value: u16) -> Self;
    fn get_address() -> TMAG5273Register;
}

/// Macro to implement the BitFieldDeviceConfiguration Trait for a given Register.
/// This is used to allow the user to use Generic Functions rather than having to write a function for each register.
///
#[macro_export]
macro_rules! impl_register {
    ($type:ty, $address:expr) => {
        impl $crate::registers::register_map::BitFieldDeviceConfiguration for $type {
            fn raw_value(&self) -> u8 {
                self.raw_value()
            }
            fn new_with_raw_value(raw_value: u8) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn get_address() -> TMAG5273Register {
                $address
            }
        }
    };
}

#[macro_export]
macro_rules! impl_dual_register {
    ($type:ty, $address:expr) => {
        impl $crate::registers::register_map::ByteFieldDeviceConfiguration for $type {
            fn raw_value(&self) -> u16 {
                self.raw_value()
            }
            fn new_with_raw_value(raw_value: u16) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn get_address() -> TMAG5273Register {
                $address
            }
        }
    };
}
