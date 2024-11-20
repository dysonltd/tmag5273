# Tests

## Summary

This folder contains all the unit tests and system integration tests for the library.
Due to the nature of the device we can not run our tests concurrently as they are
interacting with the hardware. As such we must run our tests with one thread:

```bash
cargo test -- --test-threads=1
cargo test --test connection_tests -- --test-threads=1
cargo test --test cold_start_tests -- --test-threads=1
cargo test --test setting_registers_tests --  --test-threads=1
cargo test -p tmag5273 --lib cold_start_tests  -- --test-threads=1 # Run tests within the crate
```

## Types of Tests

Currently there are two sets of tests:

- [connection_tests](./connection_tests.rs) Tests if the device is on the bus and
that it is the correct device. Ideally this should run first.
- [cold_start_tests](./cold_start_tests.rs) Tests the sensor from a 'cold' start,
this requires the sensor to be power cycled.
- [setting_register_tests](./setting_registers_tests.rs) Tests the setting and
resetting of registers on the sensor and the API for gathering.
