mod controller;
mod widget;

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
use crate::client::controller::{Dispatchable, Dispatcher};
use crate::client::widget::{WidgetBuilder, WidgetType, Widget};

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
    let mut widgets = Vec::new();

    let mut global_dispatcher = Dispatcher::new();
    // close game loop
    let run_state_handle = run_state.clone();
    global_dispatcher.add_event_binding(Box::new(controller::KeyEvent::new(
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

    let mut global_widget = WidgetBuilder::new()
        .win_type(WidgetType::GlobalInput)
        .build();
    global_widget.attach_dispatcher(global_dispatcher);
    widgets.push(global_widget);

    let mut moving_window = WidgetBuilder::new()
        .win_type(WidgetType::Generic)
        .size((3, 3))
        .build();

    let mut moving_window_controller = Dispatcher::new();
    let moving_window_handle = moving_window.get_data_handle().expect("Couldn't get window handle");
    moving_window_controller.add_event_binding(Box::new(controller::KeyEvent::new(
        KeyCode::Left,
        KeyModifiers::NONE,
        KeyEventKind::Press,
        true,
        move || -> io::Result<()> {
            let mut data_guard = moving_window_handle.lock().unwrap();
            (*data_guard).position.0 -= 1;
            drop(data_guard);
            Ok(())
        }
    )));

    let moving_window_handle = moving_window.get_data_handle().expect("Couldn't get window handle");
    moving_window_controller.add_event_binding(Box::new(controller::KeyEvent::new(
        KeyCode::Right,
        KeyModifiers::NONE,
        KeyEventKind::Press,
        true,
        move || -> io::Result<()> {
            let mut data_guard = moving_window_handle.lock().unwrap();
            (*data_guard).position.0 += 1;
            drop(data_guard);
            Ok(())
        }
    )));

    let moving_window_handle = moving_window.get_data_handle().expect("Couldn't get window handle");
    moving_window_controller.add_event_binding(Box::new(controller::KeyEvent::new(
        KeyCode::Up,
        KeyModifiers::NONE,
        KeyEventKind::Press,
        true,
        move || -> io::Result<()> {
            let mut data_guard = moving_window_handle.lock().unwrap();
            (*data_guard).position.1 -= 1;
            drop(data_guard);
            Ok(())
        }
    )));

    let moving_window_handle = moving_window.get_data_handle().expect("Couldn't get window handle");
    moving_window_controller.add_event_binding(Box::new(controller::KeyEvent::new(
        KeyCode::Down,
        KeyModifiers::NONE,
        KeyEventKind::Press,
        true,
        move || -> io::Result<()> {
            let mut data_guard = moving_window_handle.lock().unwrap();
            (*data_guard).position.1 += 1;
            drop(data_guard);
            Ok(())
        }
    )));

    moving_window.attach_dispatcher(moving_window_controller);
    widgets.push(moving_window);

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
                // window.render(&mut stdout)?;
                for widget in &widgets {
                    widget.render(&mut stdout)?;
                }
                stdout.flush()?;

                // Event dispatching
                if let Event::Key(event) = event::read()? {
                    for widget in widgets.iter_mut() {
                        widget.handle_event(&event.code, &event.kind, &event.modifiers)?;
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
                println!("\x1b[31m[ERROR]\x1b[0m {e}");
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