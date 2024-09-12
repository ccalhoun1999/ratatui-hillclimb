use crate::app::App;
use crate::ui::ui;

mod app;
mod game;
mod ui;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}
