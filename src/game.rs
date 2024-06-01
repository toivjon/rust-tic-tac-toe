use sdl2::rect::Rect;

use crate::event::Event;

/// The base structure for the whole game application.
pub struct Game {
    pub running: bool,
    pub events: Vec<Event>,
    pub cursor: (i32, i32),
    pub cell_nw: Rect,
    pub cell_nm: Rect,
    pub cell_ne: Rect,
    pub cell_mw: Rect,
    pub cell_mm: Rect,
    pub cell_me: Rect,
    pub cell_sw: Rect,
    pub cell_sm: Rect,
    pub cell_se: Rect,
}

impl Game {
    /// Create a new game instance.
    pub fn new() -> Self {
        Game {
            running: true,
            events: vec![],
            cursor: (0, 0),
            cell_nw: Rect::new(0, 0, 100, 100),
            cell_nm: Rect::new(100, 0, 100, 100),
            cell_ne: Rect::new(200, 0, 100, 100),
            cell_mw: Rect::new(0, 100, 100, 100),
            cell_mm: Rect::new(100, 100, 100, 100),
            cell_me: Rect::new(200, 100, 100, 100),
            cell_sw: Rect::new(0, 200, 100, 100),
            cell_sm: Rect::new(100, 200, 100, 100),
            cell_se: Rect::new(200, 200, 100, 100),
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
