use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
    ExecutableCommand,
    event::{self, Event, KeyCode}
};

use std::{
    error::Error,
    io,
    time::Duration,
    sync::mpsc::{self, Receiver},
    thread,
};

use snake::{
    frame::{self, Frame, new_frame, Drawable},
    render,
};

fn render_screen(render_rx: Receiver<Frame>) {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    
    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle =  std::thread::spawn(move || {
        render_screen(render_rx);
    });

    // game loop
    'gameloop: loop {
        // Initialization
        let mut curr_frame = new_frame();

        // Input handlers for the game
        while crossterm::event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                    _ => {}
                }
                
            }
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }
    //Closing and cleaning up terminal
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
