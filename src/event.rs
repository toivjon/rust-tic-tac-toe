#[derive(Clone)]
pub enum Event {
    /// Application end has been triggered.
    Quit,
    /// A mouse button has been pressed down.
    MouseButtonDown,
    /// A mouse button has been released.
    MouseButtonUp,
}
