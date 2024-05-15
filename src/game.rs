use crate::event::Event;

/// The base structure for the whole game application.
pub struct Game {
    pub running: bool,
    pub events: Vec<Event>,
}

impl Game {
    /// Create a new game instance.
    pub fn new() -> Self {
        Game {
            running: true,
            events: vec![],
        }
    }

    /// Push the provided event into the application event queue.
    pub fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }

    /// Proceed with the application logic.
    pub fn tick(&mut self) {
        for event in self.events.iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                Event::MouseButtonDown { .. } => {}
                Event::MouseButtonUp { .. } => {}
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
