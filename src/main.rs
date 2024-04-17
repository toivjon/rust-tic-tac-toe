use sdl2::event::Event;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let _window = video
        .window("Tic-Tac-Toe", 800, 600)
        .build()        
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl.event_pump()?;

    let mut running = true;
    while running {
        // Acquire events from the operating system.
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event { running = false }
        }
    }

    Ok(())
}
