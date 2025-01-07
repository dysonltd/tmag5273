# RP2040/W Test Suite

## Summary

This project is a very simple barebone test suite that uses [embedded-test](https://github.com/probe-rs/embedded-test) alongside the common tests crate found [here](../../tests-common/README.md). This allows us to test our driver on Rapsberry Pi Pico hardware using [probe-rs](https://github.com/probe-rs/probe-rs) and be able to run the regular `#[test]` statements as we would with `std` rust.

At the moment, this codebase is being used on an [RP2040W](https://thepihut.com/products/raspberry-pi-pico-w?variant=41952994754755&country=GB&currency=GBP&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&gad_source=1&gclid=Cj0KCQiAvvO7BhC-ARIsAGFyToWZjO2Q7E8yjbjFcBXdCe_0-jxRHtMKFWu4GxeYN51qq3B6_Bx4aTUaAlYKEALw_wcB), it shopuld however work with the RP2040 as well.
