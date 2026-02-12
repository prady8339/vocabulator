use crate::ui::app::App;
use ratatui::{
    Frame,
    widgets::{Block, Borders},
};

pub fn render(f: &mut Frame, _app: &App) {
    let block = Block::default().title("Placeholder").borders(Borders::ALL);

    f.render_widget(block, f.size());
}
