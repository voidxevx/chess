use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use tokio::io;

pub enum EventResult {
    Unhandled,
    Handled(io::Result<()>),
    Fallthrough(io::Result<()>),
}

pub trait Dispatchable {
    fn dispatch(self: &mut Self, code: &KeyCode, modifiers: &KeyModifiers, kind: &KeyEventKind,) -> EventResult;
}

pub struct KeyEvent <F>
where F: FnMut() -> io::Result<()> + Send + Sync + 'static {
    code: KeyCode,
    modifiers: KeyModifiers,
    kind: KeyEventKind,
    fallthrough: bool,
    closure: F,
}

impl<F> KeyEvent <F>
where F: FnMut() -> io::Result<()> + Send + Sync + 'static {
    pub fn new(code: KeyCode, modifiers: KeyModifiers, kind: KeyEventKind, fallthrough: bool, closure: F) -> Self {
        KeyEvent {
            code,
            modifiers,
            kind,
            fallthrough,
            closure
        }
    }
}

impl <F> Dispatchable for KeyEvent <F>
where F: FnMut() -> io::Result<()> + Send + Sync + 'static {
    fn dispatch(self: &mut Self, code: &KeyCode, modifiers: &KeyModifiers, kind: &KeyEventKind) -> EventResult {
        if *code == self.code && *modifiers == self.modifiers && *kind == self.kind {
            if self.fallthrough {
                EventResult::Fallthrough((self.closure)())
            } else {
                EventResult::Handled((self.closure)())
            }
        } else {
            EventResult::Unhandled
        }
    }
}



pub struct Dispatcher {
    event_bindings: Vec<Box<dyn Dispatchable>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            event_bindings: Vec::new(),
        }
    }

    pub fn add_event_binding(&mut self, event_binding: Box<dyn Dispatchable>) {
        self.event_bindings.push(event_binding);
    }

    pub fn dispatch(self: &mut Self, code: &KeyCode, kind: &KeyEventKind, mods: &KeyModifiers) -> io::Result<()> {
        for event in self.event_bindings.iter_mut() {
            match event.dispatch(code, mods, kind) {
                EventResult::Unhandled => (),
                EventResult::Handled(result) => return result,
                EventResult::Fallthrough(result) => match result {
                    Err(error) => return Err(error),
                    _ => ()
                },
            }
        }

        Ok(())
    }
}
