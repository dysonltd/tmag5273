# Tests Common

## Summary

Within this crate we have the common generic tests that we wish to run on all platforms. It had to be in its own crate due to how cargo handles sharing code. The types of tests are listed below. For more information on how to use the tests, please look [here](../tests/README.md).

## Types of Tests

Currently there are two sets of generic tests:

- [cold_start_tests](./src/generic_cold_start_tests.rs) Tests the sensor from a 'cold' start,
this requires the sensor to be power cycled.
- [setting_register_tests](./src/generic_setting_registers_tests.rs) Tests the setting and
resetting of registers on the sensor and the API for gathering.
