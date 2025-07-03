# ESP32C3 Test Suite

## Summary

This project is a very simple barebone test suite that uses [embedded-test](https://github.com/probe-rs/embedded-test) alongside the common tests [crate](../../tests-common/README.md). This allows us to test our driver on esp hardware using [probe-rs](https://github.com/probe-rs/probe-rs) and be able to run the regular `#[test]` statements as we would with `std` rust.
