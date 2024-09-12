use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    DefaultTerminal,
};

use crate::app::App;
use crate::ui::ui;

mod app;
mod ui;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut app = App::new();
    let result = run(&mut app, &mut terminal);
    ratatui::restore();
    result
}

pub fn run(app: &mut App, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}
