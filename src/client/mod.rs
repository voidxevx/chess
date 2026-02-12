mod controller;

use std::io::{self, Write, stdout};
use std::sync::{Arc, Mutex};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::board::{
    init_board,
    deinit_board,
};
use crate::client::controller::{Dispatchable, Dispatcher};

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
    ////////////////////////////////
    // APPLICATION INITIALIZATION //
    ////////////////////////////////
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


    /////////////////////
    // STATE VARIABLES //
    /////////////////////
    let run_state = Arc::new(Mutex::new(true));

    let mut dispatcher = Dispatcher::new();

    // close game loop
    let run_state_handle = run_state.clone();
    dispatcher.add_event_binding(Box::new(controller::KeyEvent::new(
        KeyCode::Char('q'),
        KeyModifiers::NONE,
        KeyEventKind::Press,
        true,
        move || -> io::Result<()> {
            let mut run_state_guard = run_state_handle.lock().unwrap();
            *run_state_guard = false;
            Ok(())
        }
    )));


    ///////////////
    // MAIN LOOP //
    ///////////////
    unsafe {
        match (|| -> io::Result<()> {
            /* Main loop */
            loop {
                // Event dispatching
                if let Event::Key(event) = event::read()? {
                    dispatcher.dispatch(&event.code, &event.kind, &event.modifiers)?;
                }

                // check the run state of the game
                let run_guard = run_state.lock().unwrap();
                if !*run_guard {
                    stdout().flush()?;
                    return Ok(());
                }
            }
            /* Main loop */
        })() /* Error handling */ {
            // failure
            Err(e) => {
                println!("[ERROR] {e}");
                clean_application()
                    .expect("\x1b[31m[ERROR]\x1b[0m Failed to clean up application");
                false
            }
            // success
            Ok(_) => {
                clean_application()
                    .expect("\x1b[31m[ERROR]\x1b[0m Failed to clean up application");
                true
            }
        }
    }
}