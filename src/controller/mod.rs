use std::collections::HashMap;
use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

type DispatchEvent = Box<dyn FnMut(KeyEventKind, KeyModifiers) -> io::Result<()>>;

pub struct PlayerController {
    event_bindings: HashMap<KeyCode, DispatchEvent>,
}

impl PlayerController {
    pub fn new() -> Self {
        PlayerController{
            event_bindings: HashMap::new(),
        }
    }

    pub fn add_event_binding<F: FnMut(KeyEventKind, KeyModifiers) -> io::Result<()> + 'static>(&mut self, code: KeyCode, event_binding: F) {
        self.event_bindings.insert(code, Box::new(event_binding));
    }

    pub fn dispatch(&mut self, key: KeyEvent) -> io::Result<()> {
        if let Some(event) = self.event_bindings.get_mut(&key.code) {
            return event(key.kind, key.modifiers);
        }
        Ok(())
    }
}