use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use crate::client::controller::{Dispatcher, EventType};
use crate::client::widget::{Widget, WidgetData};

pub struct BoardWidget {
    data: Arc<Mutex<WidgetData>>,
    controller: Option<Dispatcher>,
}

impl BoardWidget {
    pub fn new(data: Arc<Mutex<WidgetData>>) -> BoardWidget {
        BoardWidget{
            data,
            controller: None,
        }
    }
}

impl Widget for BoardWidget {
    fn handle_event(self: &mut Self, event: &EventType) -> std::io::Result<()> {
        match self.controller {
            Some(ref mut dispatcher) => {
                dispatcher.dispatch(event)
            }
            _ => Ok(())
        }
    }

    fn attach_dispatcher(self: &mut Self, dispatcher: Dispatcher) {
        self.controller = Some(dispatcher);
    }

    fn get_data_handle(self: &Self) -> Option<Arc<Mutex<WidgetData>>> {
        Some(self.data.clone())
    }

    fn render(self: &Self, _: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }
}