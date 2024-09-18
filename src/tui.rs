use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::{
    layout::{Constraint, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Circle, Line},
        Block, BorderType, Paragraph, Widget,
    },
    Frame,
};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::app::App;

#[derive(Clone, Copy)]
pub enum Event {
    // Quit,
    Error,
    Tick,
    Render,
    Key(KeyEvent),
}

pub struct Tui {
    event_tx: UnboundedSender<Event>,
    event_rx: UnboundedReceiver<Event>,
    task: JoinHandle<()>,
    tick_rate: f64,
    frame_rate: f64,
}

impl Tui {
    // pub fn new() -> Self {
    //     Tui {
    //         event_tx: mpsc::unbounded_channel(),
    //         event_rx: todo!(),
    //         task: todo!(),
    //         tick_rate: todo!(),
    //         frame_rate: todo!(),
    //     }
    // }

    pub fn start(&mut self) {
        let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
        let _event_tx = self.event_tx.clone();
        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);
            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                match evt {
                                    crossterm::event::Event::Key(key) => {
                                        if key.kind == KeyEventKind::Press {
                                            _event_tx.send(Event::Key(key)).unwrap();
                                        }
                                    },
                                    _ => {}
                                }
                            }
                            Some(Err(_)) => {
                                _event_tx.send(Event::Error).unwrap();
                            }
                            None => {},
                        }
                    },
                    _ = tick_delay => {
                        _event_tx.send(Event::Tick).unwrap();
                    },
                    _ = render_delay => {
                        _event_tx.send(Event::Render).unwrap();
                    },
                }
            }
        });
    }

    pub fn new() -> Self {
        let tick_rate = std::time::Duration::from_millis(250);

        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let _event_tx = event_tx.clone();

        let task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                let delay = interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                match evt {
                                    crossterm::event::Event::Key(key) => {
                                        if key.kind == crossterm::event::KeyEventKind::Press {
                                            event_tx.send(Event::Key(key)).unwrap();
                                        }
                                    },
                                    _ => {},
                                }
                            }
                            Some(Err(_)) => {
                                event_tx.send(Event::Error).unwrap();
                            }
                            None => {},
                        }
                    },
                    _ = delay => {
                        event_tx.send(Event::Tick).unwrap();
                    },
                }
            }
        });

        Self {
            event_tx: _event_tx,
            event_rx,
            task,
            tick_rate: 1.0,
            frame_rate: 30.0,
        }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.event_rx
            .recv()
            .await
            .ok_or(color_eyre::eyre::eyre!("Unable to get event"))
    }

    pub fn tick_rate(mut self, tick_rate: f64) -> Self {
        self.tick_rate = tick_rate;
        self
    }

    pub fn frame_rate(mut self, frame_rate: f64) -> Self {
        self.frame_rate = frame_rate;
        self
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    // let page_block = Block::default()
    //     .borders(Borders::ALL)
    //     .border_type(BorderType::Rounded)
    //     .title("Ratatui-Hillclimb");

    // frame.render_widget(page_block, frame.area());

    let chunks = Layout::vertical([Constraint::Fill(1), Constraint::Max(15)])
        .margin(1)
        .split(frame.area());

    frame.render_widget(game_canvas(app), chunks[0]);
    frame.render_widget(draw_info(app), chunks[1]);
}

fn game_canvas(app: &App) -> impl Widget + '_ {
    Canvas::default()
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Game Canvas"),
        )
        .marker(app.marker)
        .paint(|ctx| {
            // TODO: Refactor drawing the car out to a function that
            // takes the game state for location and sizes
            // Draw the car body
            let car_body_angle = app.game.get_car_body_angle();
            let car_body_vertical_angle = car_body_angle * (3.1415 / 2.0);
            let bottom_left = [app.game.get_car_body_x(), app.game.get_car_body_y()];
            let bottom_right = [
                bottom_left[0] + car_body_angle.cos() * 40.0,
                bottom_left[1] + car_body_angle.sin() * 40.0,
            ];
            let top_left = [
                bottom_left[0] + car_body_vertical_angle.cos() * 12.0,
                bottom_left[0] + car_body_vertical_angle.sin() * 12.0,
            ];
            let top_right = [
                bottom_right[0] + car_body_vertical_angle.cos() * 12.0,
                bottom_right[0] + car_body_vertical_angle.sin() * 12.0,
            ];
            ctx.draw(&Line {
                x1: bottom_left[0],
                y1: bottom_left[1],
                x2: bottom_right[0],
                y2: bottom_right[1],
                color: Color::White,
            });
            // ctx.draw(&Line {
            //     x1: top_left[0],
            //     y1: top_left[1],
            //     x2: top_right[0],
            //     y2: top_right[1],
            //     color: Color::White,
            // });
            // ctx.draw(&Line {
            //     x1: bottom_left[0],
            //     y1: bottom_left[1],
            //     x2: top_left[0],
            //     y2: top_left[1],
            //     color: Color::White,
            // });
            // ctx.draw(&Line {
            //     x1: bottom_right[0],
            //     y1: bottom_right[1],
            //     x2: top_right[0],
            //     y2: top_right[1],
            //     color: Color::White,
            // });
            ctx.draw(&Circle {
                x: app.game.get_front_wheel_x(),
                y: app.game.get_front_wheel_y(),
                radius: 6.0,
                color: Color::Black,
            });
            ctx.draw(&Circle {
                x: app.game.get_rear_wheel_x(),
                y: app.game.get_rear_wheel_y(),
                radius: 6.0,
                color: Color::Black,
            });
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
}

fn draw_info(app: &App) -> impl Widget + '_ {
    Paragraph::new(format!(
        "torque: {} x: {} y: {}",
        app.game.get_rear_wheel_torque(),
        app.game.get_car_body_x(),
        app.game.get_car_body_y()
    ))
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Game Info"),
    )

    // let todo_info = Paragraph::bordered()
    //     .border_type(BorderType::Rounded)
    //     .title("Game Info");
}
