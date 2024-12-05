# Examples

## Summary

Within this directory are a set of examples for different platforms the goal being to show the developer how to use the driver on each platform. The driver should be
platform agnostic and the API should be the same regardless of whether you are on MCU or Linux/Mac. However what will change is the setup of the I2C bus.

When working on a desktop environment the sensor will be attached via [FT232H Breakout Board](https://www.adafruit.com/product/2264)

The different set ups are shown below in the architectural diagram:

![alt text](../docs/examples/Multi%20Platform%20Driver.png)

## Installation

### How to Install (Desktop Linux)

When working with the Desktop Linux you may find you need to install the following in order to work with the [FT232H](https://www.adafruit.com/product/2264)

```bash
sudo apt update
sudo apt install libftdi1 libftdi1-dev
```

## How to Install (MacOS)

When working with MacOS you will need to install the following packages:

```bash
brew install libftdi
```

If you are having trouble it may be worth looking into:

- [CircuitPython Libraries on Any Computer with FT232H](https://learn.adafruit.com/circuitpython-on-any-computer-with-ft232h/mac-osx)

or subsequently brew might not install correctly the libftdi library. It may be worth unlinking and linking the library using the following command:

```bash
brew unlink libftdi && brew link libftdi
```

## Optional Dependencies

When listing usb devices you might find ```lsusb``` useful to which you will need to do the following:

### Linux

```bash
sudo apt update
sudo apt install lsusb
```

### MacOS

```bash
brew install lsusb
```

## How to Run

When running the examples you can choose whether or not you are using the default features or not. The default in this case is using the [F232H breakout Board](https://www.adafruit.com/product/2264). If for example you are on an embedded linux target such as [Raspberry Pi 4](https://thepihut.com/products/raspberry-pi-4-model-b?srsltid=AfmBOoolrtsYiOQS76-MPYKOBSdasCelv9UJTsQdYcnP0x3TWljbWtMN) you can use the `rpi` feature that will use the on board i2c hardware. This will swap the `setup_i2c` method and allow the examples to work as expected.

```bash
cargo build
cargo run --example example_1_basic_readings # Choose your example to run here
cargo run --example example_1_basic_readings --no-default-features --features rpi # Run on Raspberry Pi using I2C
```

### Running a BareMetal Example

As the library adheres to the [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/), the library should be able to run in `no-std` environment such as baremetal. A [esp32c3](https://github.com/esp-rs/esp-rust-board)example is shown [here](./esp32-c3/src/main.rs). Due to it using a baremetal environment its recommended to go to that directory and call cargo run from within it and flash the code on to the device.

### SparkFun Examples

The examples provided for `std` targets are heavily based off the corresponding Arduino/C++ Library written by Sparkfun linked [here](https://github.com/sparkfun/SparkFun_TMAG5273_Arduino_Library).

### F232H on MAC

When working with the [FT32H](https://www.adafruit.com/product/2264), you must find the right device id, this can be done by using `lsusb`:

```bash
❯ lsusb
Bus 003 Device 001: ID 1e91:4002 1e91 OWC Thunderbolt 3 Dock SD Card Reader  Serial: 000000001616
Bus 003 Device 002: ID 1e91:4001 1e91 OWC Thunderbolt 3 Audio Device 
Bus 003 Device 007: ID 0403:6014 Future Technology Devices International Limited Composite Device # This is the device here!!
Bus 000 Device 001: ID 1d6b:XHCI
XHCI
XHCI
1100 Linux Foundation USB 3.1 Bus 
Bus 000 Device 001: ID 1d6b:1100
XHCI Linux Foundation USB 3.0 Bus 
```

These numbers should match the numbers inside [here](../utils/src/lib.rs).
