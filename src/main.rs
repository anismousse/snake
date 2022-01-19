use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
    ExecutableCommand,
    event::{self, Event, KeyCode}
};

use std::{
    error::Error,
    io,
    time::Duration
    // thread
};

fn main() -> Result<(), Box<dyn Error>> {
    
    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // game loop
    'gameloop: loop {
        while crossterm::event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                    _ => {}
                }
                
            }
        }
    }
    //Closing and cleaning up terminal
    println!("Hello, world!");
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
