use crate::event::Event;

/// The base structure for the whole game application.
pub struct Game {
    pub running: bool,
    pub events: Vec<Event>,
    pub cursor: (i32, i32),
}

impl Game {
    /// Create a new game instance.
    pub fn new() -> Self {
        Game {
            running: true,
            events: vec![],
            cursor: (0, 0),
        }
    }

    /// Push the provided event into the application event queue.
    pub fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }

    /// Proceed with the application logic.
    pub fn tick(&mut self) {
        let events = self.events.clone();
        self.events.clear();
        for event in events {
            match event {
                Event::Quit { .. } => self.running = false,
                Event::MouseButtonDown(btn) => println!("Mouse button {:?} down", btn),
                Event::MouseButtonUp(btn) => println!("Mouse button {:?} up", btn),
                Event::MouseMotion(x, y) => self.cursor = (x, y),
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
