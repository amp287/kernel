use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut build = Build::new();

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // rebuild if `asm.s` changed
    println!("cargo:rerun-if-changed=src/interrupt.s"); 
    println!("cargo:rerun-if-changed=src/start.s"); 

    // put `link.x` in the build directory
    File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;

    println!("cargo:rerun-if-changed=link.x");

    // assemble the `asm.s` file
    build.file("src/start.s");
    build.file("src/interrupt.s");
    println!("Using compiler: {:?}", build.get_compiler());
    build.flag("--target=aarch64-unknown-elf").compile("kernel_assembly");

    Ok(())
}