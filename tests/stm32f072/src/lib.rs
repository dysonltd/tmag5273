#![no_std]
/// This module is a simple setup method and type declarations that can be shared across the tests, reducing boiler plate code.
pub mod initialise {
    use embassy_stm32::{i2c::I2c, mode::Blocking, time::Hertz};
    pub type Stm32I2c = embassy_stm32::i2c::I2c<'static, Blocking>;
    pub fn initialise() -> Stm32I2c {
        let p = embassy_stm32::init(Default::default());
        let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(400_000), Default::default());
        i2c
    }
}
