#![no_std]
/// This module is a simple setup method and type declarations that can be shared across the tests, reducing boiler plate code.
pub mod initialise {
    use defmt_rtt as _;
    use fugit::RateExtU32;
    use rp_pico::{
        hal::{
            self,
            clocks::ClockSource,
            gpio::{
                self,
                bank0::{Gpio18, Gpio19},
                FunctionI2c, Pin, PullUp,
            },
            i2c::I2C,
        },
        pac::{self, I2C1},
    };
    pub type PicoI2c = I2C<
        I2C1,
        (
            Pin<Gpio18, FunctionI2c, PullUp>,
            Pin<Gpio19, FunctionI2c, PullUp>,
        ),
    >;
    pub fn initialise() -> PicoI2c {
        let mut pac = pac::Peripherals::take().unwrap();
        let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        let clocks = hal::clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();

        // The single-cycle I/O block controls our GPIO pins
        let sio = hal::sio::Sio::new(pac.SIO);

        // Set the pins to their default state
        let pins = gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // Create the I²C driver, using the two pre-configured pins. This will fail
        // at compile time if the pins are in the wrong mode, or if this I²C
        // peripheral isn't available on these pins!

        hal::i2c::I2C::new_controller(
            pac.I2C1,
            pins.gpio18.reconfigure(),
            pins.gpio19.reconfigure(),
            400.kHz(),
            &mut pac.RESETS,
            clocks.system_clock.get_freq(),
        )
    }
}
