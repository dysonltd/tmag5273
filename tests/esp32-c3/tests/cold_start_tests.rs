#![no_std]
#![no_main]

/// Sets up the logging before entering the test-body, so that embedded-test internal logs (e.g. Running Test <...>)  can also be printed.
/// Note: you can also inline this method in the attribute. e.g. `#[embedded_test::tests(setup=rtt_target::rtt_init_log!())]`
fn setup_log() {
    #[cfg(feature = "log")]
    rtt_target::rtt_init_log!();
    #[cfg(feature = "defmt")]
    rtt_target::rtt_init_defmt!();
}

#[cfg(test)]
#[embedded_test::tests]
mod tests {

    #[init]
    fn init() {
        // This function is called once before all tests
    }

    #[test]
    fn always_passes() {
        assert!(true);
    }
}
