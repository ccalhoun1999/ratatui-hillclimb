use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    text::Line,
    widgets::{
        canvas::{Canvas, Circle, Map, MapResolution, Rectangle},
        Block, BorderType, Borders, Widget,
    },
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    // let page_block = Block::default()
    //     .borders(Borders::ALL)
    //     .border_type(BorderType::Rounded)
    //     .title("Ratatui-Hillclimb");

    // frame.render_widget(page_block, frame.area());

    let chunks = Layout::vertical([Constraint::Fill(1), Constraint::Max(15)])
        .margin(1)
        .split(frame.area());

    let todo_info = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("game info");

    frame.render_widget(app.map_canvas(), chunks[0]);
    frame.render_widget(todo_info, chunks[1]);
}

// TODO: this stuff should be raw funcs in ui but im struggling to get
// canvas to work outside of an impl
impl App {
    fn map_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Game Canvas"),
            )
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 40.0,
                    height: 12.0,
                    color: Color::White,
                });
                ctx.draw(&Circle {
                    color: Color::Black,
                    radius: 6.0,
                    x: 0.0,
                    y: 0.0,
                });
                ctx.draw(&Circle {
                    color: Color::Black,
                    radius: 6.0,
                    x: 40.0,
                    y: 0.0,
                });
                // ctx.draw(&Map {
                //     color: Color::Green,
                //     resolution: MapResolution::High,
                // });
                // ctx.print(self.x, -self.y, "You are here".yellow());
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
    }
}
