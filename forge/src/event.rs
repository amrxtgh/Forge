#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    A,
    W,
    S,
    D,
    Escape,
}

#[derive(Debug, Clone)]
pub enum Event {
    WindowClose,
    WindowResize { width: u32, height: u32 },
    WindowFocused,

    KeyPressed { key: KeyCode, repeat_count: u32 },
    KeyReleased { key: KeyCode },

    MouseEvent { x: f32, y: f32 },
    MouseButtonPressed { button: u8 },
    MouseButtonReleased { button: u8 },
    MouseScrolled { x_offset: f32, y_offset: f32 },
}

pub struct EventDispatcher<'a> {
    event: &'a mut Event,
    handled: bool,
}

impl<'a> EventDispatcher<'a> {
    pub fn new(event: &'a mut Event) -> Self {
        Self {
            event,
            handled: false,
        }
    }
    pub fn dispatch<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&Event) -> bool,
    {
        if !self.handled {
            self.handled = f(self.event)
        }
        self.handled
    }
}
