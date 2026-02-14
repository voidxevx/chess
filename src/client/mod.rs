mod controller;
mod widget;
mod board_widget;
mod widget_build;

use std::io::{self, Write, stdout};
use std::sync::{Arc, Mutex};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::{execute, ExecutableCommand, QueueableCommand};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use crate::board::{
    init_board,
    deinit_board,
};
use crate::client::controller::{Dispatchable, Dispatcher, EventType, KeyEvent};
use crate::client::widget::{Widget};
use crate::client::widget_build::{WidgetType, WindgetBuilder};

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn clean_application() -> io::Result<()> {
    // deinitialize board
    deinit_board();

    // disable terminal raw mode.
    disable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0), Show, LeaveAlternateScreen)?;
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
    if local {
        println!("Running local");
    }

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
    let mut widgets: Vec<Box<dyn Widget>> = Vec::new();

    // Global input events
    let mut global = WindgetBuilder::new(WidgetType::GlobalIn).build();
    let mut global_dispatcher = Dispatcher::new();
    let state_handle = run_state.clone();
    KeyEvent::new(
        KeyCode::Char('q'),
        KeyModifiers::NONE,
        KeyEventKind::Press,
        false,
        move || -> io::Result<()> {
            let mut state_guard = state_handle.lock().unwrap();
            *state_guard = false;
            Ok(())
        }
    ).bind(&mut global_dispatcher);
    global.attach_dispatcher(global_dispatcher);
    widgets.push(global);

    ///////////////
    // MAIN LOOP //
    ///////////////
    let mut stdout = stdout();
    unsafe {
        match (|| -> io::Result<()> {
            execute!(io::stdout(), Hide, EnterAlternateScreen)?;
            /* Main loop */
            loop {
                execute!(stdout, Clear(ClearType::All))?;
                for widget in &widgets {
                    widget.render(&mut stdout)?;
                }
                stdout.flush()?;

                // Event dispatching
                if let Event::Key(event) = event::read()? {
                    let event = EventType::Key(event.code, event.modifiers, event.kind);
                    for widget in widgets.iter_mut() {
                        widget.handle_event(&EventType::Update)?;
                        widget.handle_event(&event)?;
                    }
                } else {
                    for widget in widgets.iter_mut() {
                        widget.handle_event(&EventType::Update)?;
                    }
                }



                // check the run state of the game
                let run_state_guard = run_state.lock().unwrap();
                if !*run_state_guard {
                    stdout.flush()?;
                    return Ok(());
                }

            }
            /* Main loop */
        })() /* Error handling */ {
            // failure
            Err(e) => {
                write!(stdout, "\x1b[31m[ERROR]\x1b[0m {}", e).unwrap();
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