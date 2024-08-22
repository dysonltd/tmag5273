use super::register_map::TMAG5273Register;
use super::sensor_config_1::SensorConfig1Register;
use super::sensor_config_2::SensorConfig2Register;
use super::{device_config_1::DeviceConfig1Register, device_config_2::DeviceConfig2Register};
use crate::impl_dual_register;
use bitbybit::bitfield;

#[bitfield(u16, default = 0)]
#[derive(Debug, PartialEq)]
pub struct DeviceConfigRegisters {
    #[bits(0..=7, rw)]
    pub device_config1: DeviceConfig1Register,
    #[bits(8..=15, rw)]
    pub device_config2: DeviceConfig2Register,
}
impl_dual_register!(DeviceConfigRegisters, TMAG5273Register::DeviceConfig1);

#[bitfield(u16, default = 0)]
#[derive(Debug, PartialEq)]
pub struct SensorConfigRegisters {
    #[bits(0..=7, rw)]
    pub sensor_config1: SensorConfig1Register,
    #[bits(8..=15, rw)]
    pub sensor_config2: SensorConfig2Register,
}
impl_dual_register!(SensorConfigRegisters, TMAG5273Register::SensorConfig1);
