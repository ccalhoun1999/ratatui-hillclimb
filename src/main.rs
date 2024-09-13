use crate::app::App;
use crate::ui::ui;

mod app;
mod game;
mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = App::default().run(&mut terminal).await;
    ratatui::restore();
    result
}
