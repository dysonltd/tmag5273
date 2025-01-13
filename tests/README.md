# Tests

## Summary

This folder contains all the unit tests and system integration tests for the library. For more information on the cross platform tests, please look at the following [README](../tests-common/README.md).

### Linux/MacOs/Raspberry Pi Tests

Due to the nature of the device we can not run our tests concurrently as they are
interacting with the hardware. As such we must run our tests with one thread:

```bash
cargo test -- --test-threads=1 # Run all tests (Will need to be on a Pi with MCUs and others plugged in)
cargo test --test linux --features=std -- --test-threads=1 # Run Linux Tests using FT232H
cargo test --test linux --features=rpi -- --test-threads=1 # Run Linux Tests using Raspberry Pi I2C
```

### ESP32C3 Tests

Since we are using an MCU target we require a separate cargo workspace in order to set up the embedded test suite. More on this can be found in the following [README](./esp32c3/README.md). To run the tests on the ESP32C3 use the following code snippet:

```bash
cd tests/esp32c3
cargo test # Will run all tests one after the other
```

### Pi Pico

The same setup as the ESP32C3 is also applied to the Pi Pico. More information on this can be found in the following [README](./pipico/README.md) You can run the tests using the [Pico Probe](https://thepihut.com/products/raspberry-pi-debug-probe) like so:

```bash
cd tests/pipico
cargo test # Will run all tests one after the other
```

### STM32F072RB Tests

The same setup as the ESP32C3 is also applied to an STM32 Nucleo Board. More information on this can be found in the following [README](./stm32f072/README.md) You can run the tests like so:

```bash
cd tests/stm32f072
cargo test # Will run all tests one after the other
```
