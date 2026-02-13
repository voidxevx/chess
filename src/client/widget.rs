use std::io;
use std::io::{ErrorKind, Write};
use std::sync::{Arc, Mutex};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use crossterm::{execute, QueueableCommand};
use crossterm::cursor::MoveTo;
use crate::client::controller::{Dispatchable, Dispatcher};

pub trait Widget {
    fn handle_event(&mut self, code: &KeyCode, kind: &KeyEventKind, mods: &KeyModifiers) -> io::Result<()>;

    fn attach_dispatcher(&mut self, dispatcher: Dispatcher);

    fn get_data_handle(&self) -> Option<Arc<Mutex<WidgetData>>>;

    fn render(&self, target: &mut impl Write) -> io::Result<()>;
}

pub struct WidgetData {
    pub size: (u16, u16),
    pub position: (u16, u16),
    pub title: Box<String>,
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
    fn get_data_handle(&self) -> Option<Arc<Mutex<WidgetData>>> {
        Some(self.data.clone())
    }
    fn render(&self, target: &mut impl Write) -> io::Result<()> {
        Ok(())
    }
    fn handle_event(&mut self, code: &KeyCode, kind: &KeyEventKind, mods: &KeyModifiers) -> io::Result<()> {
        match self.controller {
            Some(ref mut dispatcher) => {
                dispatcher.dispatch(code, kind ,mods)
            }
            _ => Ok(())
        }
    }
    fn attach_dispatcher(&mut self, dispatcher: Dispatcher) {
        self.controller = Some(dispatcher);
    }
}

pub struct GenericWindow {
    data: Arc<Mutex<WidgetData>>,
    controller: Option<Dispatcher>,
}

impl GenericWindow {
    pub fn new(data: Arc<Mutex<WidgetData>>) -> GenericWindow {
        GenericWindow {
            data,
            controller: None,
        }
    }
}

impl Widget for GenericWindow {

    fn handle_event(&mut self, code: &KeyCode, kind: &KeyEventKind, mods: &KeyModifiers) -> io::Result<()> {
        match &mut self.controller {
            Some(dispatcher) => dispatcher.dispatch(code, kind, mods),
            None => Ok(())
        }
    }

    fn attach_dispatcher(&mut self, dispatcher: Dispatcher) {
        self.controller = Some(dispatcher);
    }

    fn get_data_handle(&self) -> Option<Arc<Mutex<WidgetData>>> {
        Some(self.data.clone())
    }

    fn render(&self, target: &mut impl Write) -> io::Result<()> {
        let data_guard = self.data.lock().unwrap();
        let (w, h) = data_guard.size;
        let (x_pos, y_pos) = data_guard.position;
        let title = &data_guard.title;

        execute!(target, MoveTo(x_pos, y_pos))?;
        // draw top
        let top_length = w as usize - (title.len() + 2);
        write!(target, "┌{}{}┐", title, &"─".repeat(top_length))?;

        // draw body
        for y in 0..h - 2 {
            execute!(target, MoveTo(x_pos, y_pos + y + 1))?;
            write!(target, "│")?;
            for _ in 0..w - 2
            {
                write!(target, " ")?;
            }
            write!(target, "│")?;
        }

        // draw bottom
        execute!(target, MoveTo(x_pos, y_pos + h - 1))?;
        write!(target, "└{}┘", &"─".repeat(w as usize - 2))?;
        Ok(())
    }
}

pub enum ActiveWidget {
    None,
    Generic(GenericWindow),
    GlobalInput(GlobalInput),
}

pub enum WidgetType {
    None,
    Generic,
    GlobalInput,
}

impl Widget for ActiveWidget {
    fn handle_event(&mut self, code: &KeyCode, kind: &KeyEventKind, mods: &KeyModifiers) -> io::Result<()> {
        match self {
            ActiveWidget::Generic(win) => win.handle_event(code, kind, mods),
            ActiveWidget::GlobalInput(win) => win.handle_event(code, kind, mods),
            _ => Err(io::Error::new(ErrorKind::Other, "Null window"))
        }
    }

    fn attach_dispatcher(&mut self, dispatcher: Dispatcher) {
        match self {
            ActiveWidget::Generic(win) => win.attach_dispatcher(dispatcher),
            ActiveWidget::GlobalInput(win) => win.attach_dispatcher(dispatcher),
            _ => ()
        }
    }

    fn get_data_handle(&self) -> Option<Arc<Mutex<WidgetData>>> {
        match self {
            ActiveWidget::Generic(win) => win.get_data_handle(),
            ActiveWidget::GlobalInput(win) => win.get_data_handle(),
            _ => None
        }
    }

    fn render(&self, target: &mut impl Write) -> io::Result<()> {
        match self {
            ActiveWidget::Generic(win) => win.render(target),
            ActiveWidget::GlobalInput(win) => win.render(target),
            _ => Err(io::Error::new(ErrorKind::Other, "Null window"))
        }
    }
}


pub struct WidgetBuilder {
    data: WidgetData,
    win_type: WidgetType,
}

impl WidgetBuilder {
    pub fn new() -> WidgetBuilder {
        WidgetBuilder {
            data: WidgetData{ size: (0, 0), position: (0, 0), title: Box::new("".to_string()) },
            win_type: WidgetType::None,
        }
    }

    pub fn win_type(mut self, win_type: WidgetType) -> WidgetBuilder {
        self.win_type = win_type;
        self
    }

    pub fn size(mut self, size: (u16, u16)) -> WidgetBuilder {
        self.data.size = size;
        self
    }

    pub fn position(mut self, position: (u16, u16)) -> WidgetBuilder {
        self.data.position = position;
        self
    }

    pub fn title(mut self, title: String) -> WidgetBuilder {
        self.data.title = Box::new(title);
        self
    }

    pub fn build(self) -> ActiveWidget {
        match self.win_type {
            WidgetType::Generic =>
                ActiveWidget::Generic(GenericWindow::new(Arc::new(Mutex::new(self.data)))),
            WidgetType::GlobalInput =>
                ActiveWidget::GlobalInput(GlobalInput::new(Arc::new(Mutex::new(self.data)))),
            WidgetType::None => ActiveWidget::None
        }
    }
}