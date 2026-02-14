use std::io;
use std::io::{Error, ErrorKind, Write};
use std::sync::{Arc, Mutex};
use crate::client::controller::{Dispatchable, Dispatcher, EventType};

pub trait Widget {
    fn handle_event(self: &mut Self, event: &EventType) -> io::Result<()>;

    fn attach_dispatcher(self: &mut Self, dispatcher: Dispatcher);

    fn get_data_handle(self: &Self) -> Option<Arc<Mutex<WidgetData>>>;

    fn render(self: &Self, target: &mut dyn Write) -> io::Result<()>;
}


pub struct WidgetData {
    pub size: (u16, u16),
    pub position: (u16, u16),
    pub title: Box<String>,
    pub visible: bool,
}







pub struct GlobalInput {
    data: Arc<Mutex<WidgetData>>,
    controller: Option<Dispatcher>,
}

impl GlobalInput {
    pub fn new(data: Arc<Mutex<WidgetData>>) -> GlobalInput {
        GlobalInput{
            data,
            controller: None,
        }
    }
}

impl Widget for GlobalInput {
    fn handle_event(&mut self, event: &EventType) -> io::Result<()> {
        match self.controller {
            Some(ref mut dispatcher) => {
                dispatcher.dispatch(event)
            }
            _ => Err(Error::new(ErrorKind::Other, "No dispatcher"))
        }
    }
    fn attach_dispatcher(&mut self, dispatcher: Dispatcher) {
        self.controller = Some(dispatcher);
    }
    fn get_data_handle(&self) -> Option<Arc<Mutex<WidgetData>>> {
        Some(self.data.clone())
    }
    fn render(&self, _: &mut dyn Write) -> io::Result<()> {
        Ok(())
    }
}

pub struct NullWidget {}

impl Widget for NullWidget {
    fn handle_event(self: &mut Self, _: &EventType) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "Null widget"))
    }

    fn attach_dispatcher(self: &mut Self, _: Dispatcher) {}

    fn get_data_handle(self: &Self) -> Option<Arc<Mutex<WidgetData>>> {
        None
    }

    fn render(&self, _target: &mut dyn Write) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "Null widget"))
    }
}