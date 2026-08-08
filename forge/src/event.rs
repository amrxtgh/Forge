#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    A,
    W,
    S,
    D,
    Escape,
}

#[derive(Debug, Clone)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct KeyPressedEvent {
    pub key: KeyCode,
    pub repeat_count: u32,
}

#[derive(Debug, Clone)]
pub struct KeyReleasedEvent {
    pub key: KeyCode,
}

#[derive(Debug, Clone)]
pub struct MouseMovedEvent {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct MouseButtonPressedEvent {
    pub button: u8,
}

#[derive(Debug, Clone)]
pub struct MouseButtonReleasedEvent {
    pub button: u8,
}

#[derive(Debug, Clone)]
pub struct MouseScrolledEvent {
    pub x_offset: f32,
    pub y_offset: f32,
}

#[derive(Debug, Clone)]
pub enum Event {
    WindowClose,
    WindowResize(WindowResizeEvent),
    WindowFocused,

    KeyPressed(KeyPressedEvent),
    KeyReleased(KeyReleasedEvent),

    MouseMoved(MouseMovedEvent),
    MouseButtonPressed(MouseButtonPressedEvent),
    MouseButtonReleased(MouseButtonReleasedEvent),
    MouseScrolled(MouseScrolledEvent),
}

pub trait AsVariant {
    fn as_variant(event: &Event) -> Option<&Self>;
}

// TODO: A quick macro could automate this boilerplate if variants scale
impl AsVariant for WindowResizeEvent {
    fn as_variant(event: &Event) -> Option<&Self> {
        match event vent {
                    Event::WindowClos{
            Event::WindowResize(e) => Some(e),
            _ => None,
        }
    }
}

impl AsVariant for KeyPressedEvent {
    fn as_variant(event: &Event) -> Option<&Self> {
        match event {
            Event::KeyPressed(e) => Some(e),
            _ => None,
        }
    }
}

impl AsVariant for KeyReleasedEvent {
    fn as_variant(event: &Event) -> Option<&Self> {
        match event {
            Event::KeyReleased(e) => Some(e),
            _ => None,
        }
    }
}

impl AsVariant for MouseMovedEvent {
    fn as_variant(event: &Event) -> Option<&Self> {
        match event {
            Event::MouseMoved(e) => Some(e),
            _ => None,
        }
    }
}

impl AsVariant for MouseButtonPressedEvent {
    fn as_variant(event: &Event) -> Option<&Self> {
        match event {
            Event::MouseButtonPressed(e) => Some(e),
            _ => None,
        }
    }
}

impl AsVariant for MouseButtonReleasedEvent {
    fn as_variant(event: &Event) -> Option<&Self> {
        match event {
            Event::MouseButtonReleased(e) => Some(e),
            _ => None,
        }
    }
}

impl AsVariant for MouseScrolledEvent {
    fn as_variant(event: &Event) -> Option<&Self> {
        match event {
            Event::MouseScrolled(e) => Some(e),
            _ => None,
        }
    }
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
    pub fn dispatch<T, F>(&mut self, mut f: F) -> bool
    where
        T: AsVariant,
        F: FnMut(&T) -> bool,
    {
        if !self.handled
            && let Some(payload) = T::as_variant(self.event)
        {
            self.handled = f(payload);
        }
        self.handled
    }
}
