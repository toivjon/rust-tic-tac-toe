use rust_tic_tac_toe::game::Game;
use sdl2::event::Event;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let _window = video
        .window("Tic-Tac-Toe", 800, 600)
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl.event_pump()?;

    let mut game = Game::new();
    while game.running {
        // Acquire events from the operating system.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => game.running = false,
                Event::MouseButtonDown { .. } => (),
                Event::MouseButtonUp { .. } => (),
                _ => {}
            }
        }
    }

    Ok(())
}
