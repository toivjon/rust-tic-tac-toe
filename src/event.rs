use sdl2::mouse::MouseButton;

#[derive(Clone)]
pub enum Event {
    /// Application end has been triggered.
    Quit,
    /// A mouse button has been pressed down.
    MouseButtonDown(MouseButton),
    /// A mouse button has been released.
    MouseButtonUp(MouseButton),
    /// A mouse has been moved.
    MouseMotion(i32, i32),
}
