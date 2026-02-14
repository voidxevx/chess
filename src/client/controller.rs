use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use tokio::io;

pub enum EventResult {
    Unhandled,
    Handled(io::Result<()>),
    Fallthrough(io::Result<()>),
}

pub enum EventType {
    None,
    Key(KeyCode, KeyModifiers, KeyEventKind),
    Update,
}

pub trait Dispatchable {
    fn dispatch(self: &mut Self, event: &EventType) -> EventResult;
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

    pub fn bind(self, dispatcher: &mut Dispatcher) {
        dispatcher.add_event_binding(Box::new(self));
    }
}

impl <F> Dispatchable for KeyEvent <F>
where F: FnMut() -> io::Result<()> + Send + Sync + 'static {
    fn dispatch(self: &mut Self, event: &EventType) -> EventResult {
        match event {
            EventType::Key(code, modifiers, kind) => {
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

            _ => EventResult::Unhandled,
        }
    }
}

pub struct UpdateEvent <F>
where F: FnMut() -> io::Result<()> + Send + Sync + 'static {
    fallthrough: bool,
    closure: F,
}

impl<F> UpdateEvent <F>
where F: FnMut() -> io::Result<()> + Send + Sync + 'static {
    pub fn new(fallthrough: bool, closure: F) -> Self {
        UpdateEvent {
            fallthrough,
            closure,
        }
    }
}

impl <F> Dispatchable for UpdateEvent <F>
where F: FnMut() -> io::Result<()> + Send + Sync + 'static {
    fn dispatch(self: &mut Self, event: &EventType) -> EventResult {
        match event {
            EventType::Update => {
                if self.fallthrough {
                    EventResult::Fallthrough((self.closure)())
                } else {
                    EventResult::Handled((self.closure)())
                }
            }
            _ => EventResult::Unhandled,
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

    pub fn dispatch(self: &mut Self, event: &EventType) -> io::Result<()> {
        for c_event in self.event_bindings.iter_mut() {
            match c_event.dispatch(&event) {
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
