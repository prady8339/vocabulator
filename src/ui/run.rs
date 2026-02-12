use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};

use crate::ui::screens::{custom, menu, practice, review};

use super::{
    app::{App, Screen},
    terminal::{init_terminal, restore_terminal},
};

pub fn run() -> Result<()> {
    let mut terminal = init_terminal()?;
    let mut app = App::new();

    loop {
        terminal.draw(|f| match app.current_screen {
            Screen::Menu => menu::render(f, &app),
            Screen::Practice => practice::render(f, &app),
            Screen::Review => review::render(f, &app),
            Screen::Custom => custom::render(f, &app),
        })?;

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
