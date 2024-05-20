use rust_tic_tac_toe::{event, game::Game};
use sdl2::{event::Event, pixels, rect::Rect};

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video
        .window("Tic-Tac-Toe", 800, 600)
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl.event_pump()?;

    let mut game = Game::new();
    while game.running {
        // Acquire events from the operating system.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => game.push_event(event::Event::Quit),
                Event::MouseButtonDown { mouse_btn, .. } => {
                    game.push_event(event::Event::MouseButtonDown(mouse_btn))
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    game.push_event(event::Event::MouseButtonUp(mouse_btn))
                }
                Event::MouseMotion { x, y, .. } => game.push_event(event::Event::MouseMotion(x, y)),
                _ => {}
            }
        }
        game.tick();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(pixels::Color::RGB(255, 0, 0));
        canvas.draw_rect(Rect::new(100, 100, 100, 100))?;
        canvas.present()
    }
    Ok(())
}
