use std::{io::Result, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    symbols::Marker,
    DefaultTerminal,
};

use crate::{game::Game, ui};

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
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.quitting {
            terminal.draw(|f| ui(f, self))?;

            self.handle_events()?;
            self.game.physics_loop();
        }
        Ok(())
    }

    pub fn get_ball_height(&self) -> f64 {
        self.game.get_ball_height()
    }
}

impl App {
    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != event::KeyEventKind::Release {
                    match key.code {
                        KeyCode::Char('q') => self.quitting = true,
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
