use color_eyre::eyre::Result;

use ratatui::{
    crossterm::event::{self, KeyCode},
    symbols::Marker,
    DefaultTerminal,
};

use crate::{
    game::Game,
    tui::{ui, Event, Tui},
};

pub struct App {
    pub marker: Marker,
    // pub x: f64,
    // pub y: f64,
    pub game: Game,
    quitting: bool,
}

impl Default for App {
    fn default() -> App {
        App {
            // TODO: Customize the marker, I'd like braille but
            // it breaks overlapping colors
            marker: Marker::Dot,
            // x: 0.0,
            // y: 0.0,
            game: Game::new(),
            quitting: false,
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut tui = Tui::new().tick_rate(1.0).frame_rate(30.0);
        tui.start();

        loop {
            let event = tui.next().await?;

            if let Event::Render = event.clone() {
                terminal.draw(|f| ui(f, self))?;
            }

            self.handle_events(event)?;

            self.game.physics_loop();

            if self.quitting {
                break;
            }
        }

        Ok(())
    }

    pub fn get_ball_height(&self) -> f64 {
        self.game.get_ball_height()
    }
}

impl App {
    fn handle_events(&mut self, event: Event) -> Result<()> {
        // This timeout makes sure the frame gets updated even without input
        // let timeout = Duration::from_secs_f32(1.0 / 120.0);

        if let Event::Key(key) = event {
            if key.kind != event::KeyEventKind::Release {
                match key.code {
                    KeyCode::Char('q') => self.quitting = true,
                    _ => {}
                }
            }
        }

        // if event::poll(timeout)? {
        Ok(())
    }
}
