use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};

use super::{
    app::App,
    screens::menu::render_menu,
    terminal::{init_terminal, restore_terminal},
};

pub fn run() -> Result<()> {
    let mut terminal = init_terminal()?;
    let mut app = App::new();

    loop {
        terminal.draw(|f| render_menu(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => app.select(),
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}
