use std::io::{self, Write, stdout};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::board::{
    init_board,
    deinit_board,
};

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn clean_application() -> io::Result<()> {
    // deinitialize board
    deinit_board();

    // disable terminal raw mode.
    disable_raw_mode()?;
    Ok(())
}

fn initialize_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.flush()?;
    Ok(())
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn main_loop(local: bool) -> bool {
    if !init_board() {
        println!("\x1b[31m[ERROR]\x1b[0m Board initialization failed");
        return false;
    }

    match initialize_terminal() {
        Err(e) => {
            println!("\x1b[31m[ERROR]\x1b[0m Terminal initialization failed: {e}");
            return false;
        }
        _ => ()
    }

    unsafe {
        match (|| -> io::Result<()> {
            // Main loop
            loop {
                // keyboard events
                if let Event::Key(event) = event::read()? {
                    match event.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => ()
                    }
                }


            }
        })() /* Error handling */ {
            Err(e) => {
                println!("[ERROR] {e}");
                clean_application().expect("\x1b[31m[ERROR]\x1b[0m Failed to clean up application");
                false
            }
            Ok(_) => {
                clean_application().expect("\x1b[31m[ERROR]\x1b[0m Failed to clean up application");
                true
            }
        }
    }
}