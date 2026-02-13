mod controller;
mod window;

use std::arch::x86_64::_store_mask16;
use std::io::{self, Write, stdout, Cursor};
use std::sync::{Arc, Mutex};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::{execute, ExecutableCommand, QueueableCommand};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use crate::board::{
    init_board,
    deinit_board,
};
use crate::client::controller::{Dispatchable, Dispatcher};
use crate::client::window::{GenericWindow, Window, Windowable};

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn clean_application() -> io::Result<()> {
    // deinitialize board
    deinit_board();

    // disable terminal raw mode.
    disable_raw_mode()?;
    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0), Show, LeaveAlternateScreen)?;
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

    // event dispatcher
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

    let mut window = Window::Generic(
        GenericWindow::new(
            (70, 45),
            (10, 5),
            dispatcher,
            run_state.clone(),
            "Test Window".to_string(),
        )
    );

    ///////////////
    // MAIN LOOP //
    ///////////////
    unsafe {
        match (|| -> io::Result<()> {
            let mut stdout = stdout();
            execute!(io::stdout(), Hide, EnterAlternateScreen)?;
            /* Main loop */
            loop {
                stdout.queue(Clear(ClearType::All))?;
                window.render(&mut stdout)?;
                stdout.flush()?;

                // Event dispatching
                if let Event::Key(event) = event::read()? {
                    window.handle_event(&event.code, &event.kind, &event.modifiers)?;
                }

                // check the run state of the game
                if window.should_close() {
                    stdout.flush()?;
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