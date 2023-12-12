use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // do this only if building for the device, i.e. not terminal mode
    if env::var("CARGO_CFG_TARGET_OS").ok() == Some("none".into()) {
        // Put `memory.x` in our output directory and ensure it's
        // on the linker search path.
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("memory.x"))
            .unwrap()
            .write_all(include_bytes!("memory.x"))
            .unwrap();

        println!("cargo:rustc-link-arg-bins=--nmagic");
        println!("cargo:rustc-link-arg-bins=-Tlink.x");
    }
}
