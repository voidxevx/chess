use std::io;
use std::io::{Stdout, StdoutLock, Write};
use std::process::Output;
use std::sync::{Arc, Mutex};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use crossterm::{terminal, ExecutableCommand};
use crate::client::controller::{Dispatchable, Dispatcher};

pub trait Windowable {
    fn set_size(&mut self, size: (u16, u16));
    fn get_size(&self) -> (u16, u16);
    fn handle_event(&mut self, code: &KeyCode, kind: &KeyEventKind, mods: &KeyModifiers) -> io::Result<()>;
    fn should_close(&self) -> bool;

    fn render(&self, target: &mut String) -> io::Result<()>;
}


pub struct GenericWindow {
    controller: Dispatcher,
    size: (u16, u16),
    run_state: Arc<Mutex<bool>>,
    title: String,
}

impl GenericWindow {
    pub fn new(size: (u16, u16), controller: Dispatcher, state: Arc<Mutex<bool>>, title: String) -> GenericWindow {
        GenericWindow {
            controller,
            size,
            run_state: state,
            title,
        }
    }
}

impl Windowable for GenericWindow {
    fn set_size(&mut self, size: (u16, u16)) {
        self.size = size;
    }
    fn get_size(&self) -> (u16, u16) {
        self.size
    }

    fn handle_event(&mut self, code: &KeyCode, kind: &KeyEventKind, mods: &KeyModifiers) -> io::Result<()> {
        self.controller.dispatch(code, kind, mods)
    }

    fn should_close(&self) -> bool {
        !*self.run_state.lock().unwrap()
    }

    fn render(&self, target: &mut String) -> io::Result<()> {
        target.push_str("┌");
        target.push_str(&self.title);
        Ok(())
    }
}

pub enum Window {
    Generic(GenericWindow),
}


impl Windowable for Window {
    fn set_size(&mut self, size: (u16, u16)) {
        match self {
            Window::Generic(win) => win.set_size(size)
        }
    }

    fn get_size(&self) -> (u16, u16) {
        match self {
            Window::Generic(win) => win.get_size()
        }
    }

    fn handle_event(&mut self, code: &KeyCode, kind: &KeyEventKind, mods: &KeyModifiers) -> io::Result<()> {
        match self {
            Window::Generic(win) => win.handle_event(code, kind, mods)
        }
    }

    fn should_close(&self) -> bool {
        match self {
            Window::Generic(win) => win.should_close(),
        }
    }

    fn render(&self, target: &mut String) -> io::Result<()> {
        match self {
            Window::Generic(win) => win.render(target)
        }
    }
}


