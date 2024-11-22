# Utils

## Summary

This internal crate is a utilities package designed for connecting the Embedded Targets to the Library as such it is not to be used as part of the library.

The main thing this crate offers is a generic way of setting up the I2C Bus on different Linux/MacOS targets. This is done through the `setup_i2c()` method outlined in [lib.rs](./src/lib.rs).

### Features

This crates offers essentially two features:

- std: This pulls in the [ftdi-embedded-hal](https://github.com/ftdi-rs/ftdi-embedded-hal/tree/main) crate which allows the user to use the [F232H Breakout Board](https://thepihut.com/products/adafruit-ft232h-breakout-general-purpose-usb-to-gpio-spi-i2c)

- rpi: This pulls both the [ftdi-embedded-hal](https://github.com/ftdi-rs/ftdi-embedded-hal/tree/main) crate and the [rppal](https://github.com/golemparts/rppal) crate allowing the user to select either I2C via hardware on the board or through the [FT232H Breakout Board](https://thepihut.com/products/adafruit-ft232h-breakout-general-purpose-usb-to-gpio-spi-i2c).
