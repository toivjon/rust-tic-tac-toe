/// The base structure for the whole game application.
pub struct Game {
    pub running: bool,
}

impl Game {
    /// Create a new game instance.
    pub fn new() -> Self {
        Game { running: true }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
