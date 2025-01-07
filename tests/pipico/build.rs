fn main() {
    // add linker script for embedded-test!!
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
    //link the defmt linker script
    println!("cargo:rustc-link-arg=-Tdefmt.x");
}
