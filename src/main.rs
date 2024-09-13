use crate::app::App;

mod app;
mod game;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = App::default().run(&mut terminal).await;
    ratatui::restore();
    result
}
