pub mod board;
pub mod controller;

use std::fmt::Debug;
use std::io::{self, ErrorKind, Read, Write, stdout};
use std::sync::{Arc, Mutex};
use crossterm::event;
use crate::board::{
    init_board,
    deinit_board,
};

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::controller::PlayerController;

unsafe fn clean_application() -> io::Result<()> {
    // deinitialize board
    deinit_board();

    // disable terminal raw mode.
    disable_raw_mode()?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize board
    unsafe  { if !init_board() {
        Err(io::Error::new(ErrorKind::Other, "Initialization failed"))?
    }}

    // initialize terminal state
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.flush()?;

    // main running state mutex
    let mut running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    // controller initialization
    let mut controller = Arc::new(Mutex::new(PlayerController::new()));
    let mut controller_guard = controller.lock().unwrap();

    let run_handle = running.clone();
    (*controller_guard).add_event_binding(KeyCode::Char('q'), move |kind: KeyEventKind, mods: KeyModifiers|{
        let mut run_guard = run_handle.lock().unwrap();
        *run_guard = false;
        Ok(())
    });

    drop(controller_guard);


    // main loop
     if let Err(e) = (async move || {
         loop {
            // event dispatcher
            if let Event::Key(event) = event::read()? {
                let mut controller = controller.lock().unwrap();
                controller.dispatch(event)?;
            }

            // check run state
            let mut run_guard = running.lock().unwrap();
            if !*run_guard {
                return Ok(());
            }
            drop(run_guard);
        }
     })().await {
         // runtime failure
         unsafe { clean_application() }?;
         Err(e)
     } else {
         // safe execution
        unsafe { clean_application() }?;
        Ok(())
     }
}

