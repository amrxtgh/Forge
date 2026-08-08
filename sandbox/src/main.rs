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

    // create a EventDispatcher around the event and called dispatch with closure
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

    // Opens the window and loops while running
    fn run(&mut self) {
        println!("Forge Sandbox application running");
        // create events and then event goes to Application::on_events(e)
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
