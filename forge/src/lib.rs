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

    // Mocking events
    pub fn process_events(&mut self) -> Vec<Event> {
        vec![
            Event::WindowClose,
            Event::KeyPressed {
                key: event::KeyCode::Escape,
                repeat_count: 0,
            },
        ]
    }
}
