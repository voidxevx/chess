pub mod board;
pub mod client;

// This fixes a random issue on windows related to floating point numbers -- DO NOT REMOVE
#[cfg(target_os = "windows")]
#[used]
#[unsafe(no_mangle)]
pub static _fltused: i32 = 0;