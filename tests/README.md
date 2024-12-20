# Tests

## Summary

This folder contains all the unit tests and system integration tests for the library. For more information on the cross platform tests, please look at the following [README](../tests-common/README.md).

### Linux/MacOs/Raspberry Pi Tests

Due to the nature of the device we can not run our tests concurrently as they are
interacting with the hardware. As such we must run our tests with one thread:

```bash
cargo test -- --test-threads=1 # Run all tests (Will need to be on a Pi with MCUs and others plugged in)
cargo test --test linux --features=std -- --test-threads=1 # Run Linux Tests using FTD232
cargo test --test linux --features=rpi -- --test-threads=1 #Run Linux Tests using Raspberry Pi I2C
```

### ESP32C3 Tests

Since we are using an MCU target we require a seperate cargo workspace in order to set up the embedded test suite. More on this can be found in the following [README](./esp32c3/README.md). To run the tests on the ESP32C3 use the following code snippet:

```bash
cd tests/esp32c3
cargo test # Will run all tests one after the other
```

## Test Environments

Currently we support testing on the following environments:

- [Linux](./linux.rs) This can be with both a Raspberry Pi and a Desktop Linux environment through the use of an FT232H breakout Board.

- [ESP32C3](./esp32c3/tests/) This requires an esp32c3, the one we are using in the CI is the [ESP32C3 QTPY](https://thepihut.com/products/adafruit-qt-py-esp32-pico-wifi-dev-board-with-stemma-qt-8mb-flash-2mb-psram?variant=43725054902467&country=GB&currency=GBP&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&gad_source=1&gclid=CjwKCAiAyJS7BhBiEiwAyS9uNYpjqy39hvD1gq5iRmWylSS3AsA8kcc3a7bK7BSWQwjkghoY49X0nxoCMi4QAvD_BwE).
