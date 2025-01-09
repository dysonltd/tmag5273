# STM32F072RB Test Suite

## Summary

This folder contains a test suite project using the [embassy-stm32](https://crates.io/crates/embassy-stm32) HAL. The reason this HAL was chosen over the [stm3f0xx-hal](https://crates.io/crates/stm32f0xx-hal), was due to its continued support through the Embassy project and that it supported `embedded-hal=v1.0.0`. [probe-rs](https://github.com/probe-rs/probe-rs) is used as the toolchain debugger using the onboard st-link.

## Notes

Due to the embedded-test framework and the size limitations of the [STM32F072RB](https://www.digikey.co.uk/en/products/detail/stmicroelectronics/NUCLEO-F072RB/5047984?gclsrc=aw.ds&&utm_adgroup=General&utm_source=google&utm_medium=cpc&utm_campaign=PMax%20Shopping_Product_Zombie%20SKUs&utm_term=&productid=5047984&utm_content=General&utm_id=go_cmp-17923835716_adg-_ad-__dev-c_ext-_prd-5047984_sig-CjwKCAiAhP67BhAVEiwA2E_9g_0YOhbNz6vGH5AFGYettjiugiMMnm4g4Ein_NavuaIqUj26ybKOCRoCxEcQAvD_BwE&gad_source=1&gclid=CjwKCAiAhP67BhAVEiwA2E_9g_0YOhbNz6vGH5AFGYettjiugiMMnm4g4Ein_NavuaIqUj26ybKOCRoCxEcQAvD_BwE&gclsrc=aw.ds), I found that It did not have enough flash storage to hold all of the cold_start_tests. Hence why in this suite they are split.

Running `cargo bloat --test cold_start_tests_0 --crates` revealed the following:

```bash
argo bloat --test cold_start_tests_0 --crates
   Compiling stm32f072 v0.1.0 (/Users/scott.gibb/Documents/Projects/rs-tmag5273-driver/tests/stm32f072)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.21s
    Analyzing target/thumbv6m-none-eabi/debug/deps/cold_start_tests_0-7aa7073ea1a6a38f

File  .text    Size Crate
0.8%  32.6% 26.7KiB std
0.8%  29.6% 24.3KiB embassy_stm32
0.1%   5.8%  4.7KiB stm32f072
0.1%   5.4%  4.5KiB tmag5273
0.1%   5.3%  4.3KiB serde_json_core
0.1%   4.4%  3.6KiB semihosting
0.1%   3.5%  2.9KiB defmt_rtt
0.1%   3.3%  2.7KiB serde
0.1%   2.2%  1.8KiB embedded_test
0.0%   1.9%  1.6KiB tests_common
0.0%   1.4%  1.1KiB defmt
0.0%   1.3%  1.1KiB [Unknown]
0.0%   0.6%    478B embassy_time
0.0%   0.6%    472B embassy_sync
0.0%   0.5%    396B embassy_hal_internal
0.0%   0.4%    298B heapless
0.0%   0.3%    250B cold_start_tests_0
0.0%   0.3%    226B cortex_m
0.0%   0.1%    100B hash32
0.0%   0.1%     96B arbitrary_int
0.0%   0.3%    230B And 6 more crates. Use -n N to show more.
2.5% 100.0% 82.0KiB .text section size, the file size is 3.2MiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

I believe the embedded-test and defmt framework is pulling in some part of `std` which is then filling the flash.

Further inspection with the following:

```bash
cargo bloat --test cold_start_tests_0         
   Compiling stm32f072 v0.1.0 (/Users/scott.gibb/Documents/Projects/rs-tmag5273-driver/tests/stm32f072)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
    Analyzing target/thumbv6m-none-eabi/debug/deps/cold_start_tests_0-7aa7073ea1a6a38f

File  .text    Size           Crate Name
0.1%   2.3%  1.9KiB   embassy_stm32 embassy_stm32::rcc::_version::init
0.0%   1.6%  1.3KiB             std core::str::pattern::StrSearcher::new
0.0%   1.4%  1.2KiB serde_json_core <&mut serde_json_core::ser::Serializer as serde::ser::Serializer>::serialize_str
0.0%   1.4%  1.1KiB             std core::str::count::do_count_chars
0.0%   1.3%  1.1KiB   embassy_stm32 embassy_stm32::i2c::_version::Timings::new
0.0%   1.2%   1004B   embassy_stm32 embassy_stm32::rcc::bd::LsConfig::init
0.0%   1.1%    934B             std compiler_builtins::int::specialized_div_rem::u64_div_rem
0.0%   1.0%    848B    tests_common tests_common::generic_cold_start_tests::generic_test_registers
0.0%   1.0%    828B   embassy_stm32 embassy_stm32::dma::bdma::on_irq_inner
0.0%   1.0%    812B           serde core::str::pattern::TwoWaySearcher::next
0.0%   1.0%    812B           serde core::str::pattern::TwoWaySearcher::next
0.0%   1.0%    808B   embassy_stm32 embassy_stm32::time_driver::RtcDriver::init
0.0%   1.0%    800B   embedded_test embedded_test::export::run_tests
0.0%   0.9%    784B    semihosting? <semihosting::io::error::ErrorKind as core::fmt::Debug>::fmt
0.0%   0.9%    724B             std <core::str::pattern::StrSearcher as core::str::pattern::Searcher>::next
0.0%   0.8%    692B             std core::str::slice_error_fail_rt
0.0%   0.8%    692B   embassy_stm32 embassy_stm32::i2c::_version::<impl embassy_stm32::i2c::I2c<T,TXDMA,RXDMA>>::write_internal
0.0%   0.8%    656B     semihosting semihosting::experimental::env::sys::next_from_cmdline
0.0%   0.8%    636B             std <str as core::fmt::Debug>::fmt
0.0%   0.8%    636B   embassy_stm32 embassy_stm32::i2c::_version::<impl embassy_stm32::i2c::I2c<T,TXDMA,RXDMA>>::read_internal
2.0%  78.0% 63.9KiB                 And 1087 smaller methods. Use -n N to show more.
2.5% 100.0% 82.0KiB                 .text section size, the file size is 3.2MiB
```
