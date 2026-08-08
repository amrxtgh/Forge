use forge::{
    Window,
    event::{self, Event, EventDispatcher},
};
struct Application {
    window: Window,
    running: bool,
}
impl Application {
    pub fn new() -> Self {
        Self {
            window: Window::new("Forge Engine v0.1"),
            running: true,
        }
    }

    pub fn on_event(&mut self, mut event: Event) {
        println!("Engine Received Event: {:?}", event);

        let mut dispatcher = EventDispatcher::new(&mut event);

        dispatcher.dispatch(|e| match e {
            Event::WindowClose => {
                self.on_window_close();
                true
            }
            _ => false,
        });
        dispatcher.dispatch(|e| match e {
            Event::KeyPressed {
                key: event::KeyCode::Escape,
                ..
            } => {
                println!("Escape key hit! Exiting application loop");
                self.running = false;
                true
            }
            _ => false,
        });
    }
    fn on_window_close(&mut self) -> bool {
        println!("Closing main window frame");
        self.running = false;
        true
    }

    fn run(&mut self) {
        println!("Forge Sandbox application running");
        while self.running {
            for event in self.window.process_events() {
                self.on_event(event);
            }
        }
    }
}

fn main() {
    let mut app = Application::new();
    app.run();
}
