// rust build scripting
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/root.zig");

    // run zig build command
    let status = Command::new("zig")
        .arg("build")
        .arg("-Dlib=true")
        .status()
        .expect("Unable to run zig build");

    if !status.success() {
        panic!("zig build failed");
    }

    // link the zig library
    println!("cargo:rustc-link-search=native=zig-out/lib");
    println!("cargo:rustc-link-lib=static=chess");
}