pub mod event;
use crate::event::Event;
pub mod log;

pub struct Window {
    pub title: String,
}
impl Window {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn process_events<F>(&mut self, mut event_callback: F)
    where
        F: FnMut(Event),
    {
        let mock_os_close = Event::WindowClose;
        let mock_os_input = Event::KeyPressed {
            key: event::KeyCode::Escape,
            repeat_count: 0,
        };
        event_callback(mock_os_close);
        event_callback(mock_os_input);
    }
}
