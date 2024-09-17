use color_eyre::eyre::Result;

use ratatui::{
    crossterm::event::{self, KeyCode},
    symbols::Marker,
    DefaultTerminal,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{
    game::Game,
    tui::{ui, Event, Tui},
};

#[derive(Clone, Copy)]
pub enum Action {
    None,
    Tick,
    Quit,
    Render,
    Accelerate,
    Deccelerate,
}

pub struct App {
    pub marker: Marker,
    // pub x: f64,
    // pub y: f64,
    pub game: Game,
    quitting: bool,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
}

impl Default for App {
    fn default() -> App {
        let (action_tx, action_rx) = mpsc::unbounded_channel::<Action>();
        App {
            // TODO: Customize the marker, I'd like braille but
            // it breaks overlapping colors
            marker: Marker::Dot,
            // x: 0.0,
            // y: 0.0,
            game: Game::new(),
            quitting: false,
            action_tx,
            action_rx,
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut tui = Tui::new().tick_rate(60.0).frame_rate(60.0);
        tui.start();

        loop {
            let event = tui.next().await?;

            match event {
                // Event::Quit => self.action_tx.send(Action::Quit)?,
                Event::Tick => self.action_tx.send(Action::Tick)?,
                Event::Render => self.action_tx.send(Action::Render)?,
                Event::Key(_) => {
                    let action = self.get_action(event);
                    self.action_tx.send(action.clone())?;
                }
                _ => {}
            };

            while let Ok(action) = self.action_rx.try_recv() {
                self.handle_events(action.clone());

                if let Action::Render = action {
                    terminal.draw(|f| ui(f, self))?;
                }
            }

            if self.quitting {
                break;
            }
        }

        Ok(())
    }
}

impl App {
    fn handle_events(&mut self, action: Action) {
        // This timeout makes sure the frame gets updated even without input
        // let timeout = Duration::from_secs_f32(1.0 / 120.0);

        match action {
            Action::Quit => self.quitting = true,
            Action::Tick => self.game.step_physics(),
            Action::Accelerate => self.game.apply_torque(5000.0),
            Action::Deccelerate => self.game.apply_torque(-5000.0),
            // Action::None => self.game.apply_torque(-2.0),
            _ => {}
        };

        // if event::poll(timeout)? {
        // Ok(())
    }

    fn get_action(&self, event: Event) -> Action {
        if let Event::Key(key) = event {
            if key.kind != event::KeyEventKind::Release {
                return match key.code {
                    KeyCode::Char('q') => Action::Quit,
                    KeyCode::Right => Action::Accelerate,
                    KeyCode::Left => Action::Deccelerate,
                    _ => Action::None,
                };
            }
        };
        Action::None
    }
}
