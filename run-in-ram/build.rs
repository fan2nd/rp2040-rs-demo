fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlinkrp.x");
    println!("cargo:rustc-link-arg-bins=-Tdevice.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
