use forge::{
    event::{Event, EventDispatcher, KeyCode, KeyPressedEvent, WindowResizeEvent},
};

struct Application {
    running: bool,
}
impl Application {
    pub fn on_event(&mut self, mut event: Event) {
        println!("Engine Received Event: {:?}", event);

        let mut dispatcher = EventDispatcher::new(&mut event);

        dispatcher.dispatch::<KeyPressedEvent, _>(|e| {
            println!("Key caught by dispatcher: {:?}", e.key);
            if e.key == KeyCode::Escape {
                println!("Escape key hit! Exiting application loop");
                self.running = false;
                return true; // Event is marked handled and consumed
            }
            false
        });

        dispatcher.dispatch::<WindowResizeEvent, _>(|e| {
            println!("Viewport sizing to: {}x{}", e.width, e.height);
            true
        });
    }
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Forge Engine", 1280, 720).position
}
