mod configs;
/// Public Registers for the TMAG5273
mod conversion_status;
mod device_config_1;
mod device_config_2;
mod device_id;
mod device_status;
mod i2c_address;
mod interrupt_config;
mod register_map;
mod sensor_config_1;
mod sensor_config_2;
mod temperature_config;

use crate::traits::{BitFieldDeviceConfiguration, ByteFieldDeviceConfiguration};

pub use configs::*;
pub use conversion_status::*;
pub use device_config_1::*;
pub use device_config_2::*;
pub use device_id::*;
pub use device_status::*;
use embedded_hal::i2c::{I2c, SevenBitAddress};
pub use i2c_address::*;
pub use interrupt_config::*;
pub use register_map::*;
pub use sensor_config_1::*;
pub use sensor_config_2::*;
pub use temperature_config::*;

use crate::{types::TMag5273Error, TMag5273};

impl<I2C> TMag5273<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// ### Raw Registers
    ///
    /// Generic function to set a register value on the device. The Register must implement the traits
    /// BitFieldDeviceConfiguration, in order to set it on the device.
    pub fn set_config_register<Register>(&mut self, register: Register) -> Result<(), TMag5273Error>
    where
        Register: BitFieldDeviceConfiguration,
    {
        let register_address = Register::get_address().into();
        let data = register.raw_value();
        self.i2c.write(self.address, &[register_address, data])?;
        Ok(())
    }
    /// ### Raw Registers
    ///
    /// Generic function to get a register value from the device. The Register must implement the traits
    /// BitFieldDeviceConfiguration in order to get it from the device.
    pub fn get_config_register<Register>(&mut self) -> Result<Register, TMag5273Error>
    where
        Register: BitFieldDeviceConfiguration,
    {
        let mut data: [u8; 1] = [0x00];
        let register_address = Register::get_address().into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;
        Ok(Register::new_with_raw_value(data[0]))
    }

    /// ### Raw Registers
    ///
    /// Generic function to get a register value from the device. The Register must implement the traits
    /// ByteFieldDeviceConfiguration in order to get it from the device.
    pub(crate) fn get_dual_config_register<Register>(&mut self) -> Result<Register, TMag5273Error>
    where
        Register: ByteFieldDeviceConfiguration,
    {
        let mut data: [u8; 2] = [0x00, 2];
        let register_address = Register::get_address().into();
        self.i2c
            .write_read(self.address, &[register_address], &mut data)?;
        Ok(Register::new_with_raw_value(u16::from_le_bytes(data)))
    }
}
