#![no_std]
/// This module is a simple setup method and type declarations that can be shared across the tests, reducing boiler plate code.
pub mod initialise {
    use embassy_stm32::{
        bind_interrupts,
        i2c::{self, I2c},
        mode::Blocking,
        peripherals,
        time::Hertz,
        Config,
    };
    pub type Stm32I2c = embassy_stm32::i2c::I2c<'static, Blocking>;
    pub fn initialise() -> Stm32I2c {
        bind_interrupts!(struct Irqs {
            I2C1 => i2c::EventInterruptHandler<peripherals::I2C1>, i2c::ErrorInterruptHandler<peripherals::I2C1>;
        });
        let config = Config::default();
        let p = embassy_stm32::init(config);
        let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(400_000), Default::default());
        i2c
    }
}
