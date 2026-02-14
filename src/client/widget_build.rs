use std::sync::{Arc, Mutex};
use crate::client::board_widget::BoardWidget;
use crate::client::widget::{GlobalInput, NullWidget, Widget, WidgetData};

pub enum WidgetType {
    None,
    GlobalIn,
    Board
}

pub struct WindgetBuilder {
    data: WidgetData,
    widget_type: WidgetType,
}

impl WindgetBuilder {
    pub fn new(widget_type: WidgetType) -> WindgetBuilder {
        WindgetBuilder {
            data: WidgetData {
                size: (0, 0),
                position: (0, 0),
                title: Box::new("".to_string()),
                visible: false,
            },
            widget_type,
        }
    }

    pub fn size(mut self, size: (u16, u16)) -> WindgetBuilder {
        self.data.size = size;
        self
    }

    pub fn position(mut self, pos: (u16, u16)) -> WindgetBuilder {
        self.data.position = pos;
        self
    }

    pub fn title(mut self, title: String) -> WindgetBuilder {
        self.data.title = Box::new(title);
        self
    }

    pub fn visible(mut self, visible: bool) -> WindgetBuilder {
        self.data.visible = visible;
        self
    }

    pub fn build(self) -> Box<dyn Widget> {
        match self.widget_type {
            WidgetType::None => Box::new(NullWidget{}),
            WidgetType::GlobalIn => Box::new(GlobalInput::new(Arc::new(Mutex::new(self.data)))),
            WidgetType::Board => Box::new(BoardWidget::new(Arc::new(Mutex::new(self.data)))),
        }
    }
}