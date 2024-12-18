# Tests

## Summary

This folder contains all the unit tests and system integration tests for the library.
Due to the nature of the device we can not run our tests concurrently as they are
interacting with the hardware. As such we must run our tests with one thread:

```bash
cargo test -- --test-threads=1 # Run all tests (Will need to be on a Pi with MCUs and others plugged in)
cargo test --test linux --features=std -- --test-threads=1 # Run Linux Tests using FTD232
cargo test --test linux --features=rpi -- --test-threads=1 #Run Linux Tests using Raspberry Pi I2C
```

## Types of Tests

Currently there are two sets of generic tests:

- [cold_start_tests](./generic_cold_start_tests.rs) Tests the sensor from a 'cold' start,
this requires the sensor to be power cycled.
- [setting_register_tests](./generic_setting_registers_tests.rs) Tests the setting and
resetting of registers on the sensor and the API for gathering.

## Test Environments

Currently we support testing on the following environments:

- [Linux](./linux/) This can be with both a Raspberry Pi and a Desktop Linux environment through the use of an FT232H breakout Board.
