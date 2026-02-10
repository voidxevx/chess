use std::io::{self, Write, stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::board::{
    init_board,
    deinit_board,
};

unsafe fn clean_application() -> io::Result<()> {
    // deinitialize board
    deinit_board();

    // disable terminal raw mode.
    disable_raw_mode()?;
    Ok(())
}

fn initialize_termianl() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.flush()?;
    Ok(())
}

unsafe extern "C" fn main_loop() -> bool {
    if !init_board() {
        println!("[ERROR] Board initialization failed");
        return false;
    }

    match initialize_termianl() {
        Err(e) => {
            println!("[ERROR] Terminal initialization failed");
            return false;
        }
        _ => ()
    }

    true
}