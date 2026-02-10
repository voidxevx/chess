// rust build scripting
use std::process::Command;

#[cfg(target_os = "windows")]
fn zig_compile() {
    let status = Command::new("zig")
    .arg("build")
    .arg("-Dlib=true")
    .arg("-Dtarget=x86_64-windows-msvc")
    .status()
    .expect("Unable to run zig build");

    if !status.success() {
        panic!("zig build failed");
    }
}

#[cfg(not(target_os = "windows"))]
fn zig_compile() {
    let status = Command::new("zig")
    .arg("build")
    .arg("-Dlib=true")
    .status()
    .expect("Unable to run zig build");

    if !status.success() {
        panic!("zig build failed");
    }
}

fn main() {
    // rerun if ant rust or zig files change
    println!("cargo:rerun-if-changed=src/");

    // run zig build command
    zig_compile();

    // link the zig library
    println!("cargo:rustc-link-search=native=zig-out/lib");
    println!("cargo:rustc-link-lib=static=chess");
}