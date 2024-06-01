use sdl2::rect::Rect;

use crate::event::Event;

/// The width and height of a single cell in the grid.
const CELL_SIZE: u32 = 200;
/// The size being reserved for each cell.
const CELL_MARGIN: i32 = CELL_SIZE as i32;

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
            cell_nw: Rect::new(0, 0, CELL_SIZE, CELL_SIZE),
            cell_nm: Rect::new(CELL_MARGIN, 0, CELL_SIZE, CELL_SIZE),
            cell_ne: Rect::new(2 * CELL_MARGIN, 0, CELL_SIZE, CELL_SIZE),
            cell_mw: Rect::new(0, CELL_MARGIN, CELL_SIZE, CELL_SIZE),
            cell_mm: Rect::new(CELL_MARGIN, CELL_MARGIN, CELL_SIZE, CELL_SIZE),
            cell_me: Rect::new(2 * CELL_MARGIN, CELL_MARGIN, CELL_SIZE, CELL_SIZE),
            cell_sw: Rect::new(0, 2 * CELL_MARGIN, CELL_SIZE, CELL_SIZE),
            cell_sm: Rect::new(CELL_MARGIN, 2 * CELL_MARGIN, CELL_SIZE, CELL_SIZE),
            cell_se: Rect::new(2 * CELL_MARGIN, 2 * CELL_MARGIN, CELL_SIZE, CELL_SIZE),
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
