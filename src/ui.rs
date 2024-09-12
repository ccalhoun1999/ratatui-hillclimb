use ratatui::{
    layout::{Constraint, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Circle, Rectangle},
        Block, BorderType, Paragraph, Widget,
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

    frame.render_widget(game_canvas(app), chunks[0]);
    frame.render_widget(draw_info(app), chunks[1]);
}

fn game_canvas(app: &App) -> impl Widget + '_ {
    // -> impl Widget + '_ {

    Canvas::default()
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Game Canvas"),
        )
        .marker(app.marker)
        .paint(|ctx| {
            let car_y = app.get_ball_height();
            // TODO: Refactor drawing the car out to a function that
            // takes the game state for location and sizes
            ctx.draw(&Rectangle {
                x: 0.0,
                y: car_y,
                width: 40.0,
                height: 12.0,
                color: Color::White,
            });
            ctx.draw(&Circle {
                x: 0.0,
                y: car_y,
                radius: 6.0,
                color: Color::Black,
            });
            ctx.draw(&Circle {
                x: 40.0,
                y: car_y,
                radius: 6.0,
                color: Color::Black,
            });
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
}

fn draw_info(app: &App) -> impl Widget + '_ {
    Paragraph::new(app.get_ball_height().to_string()).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Game Info"),
    )

    // let todo_info = Paragraph::bordered()
    //     .border_type(BorderType::Rounded)
    //     .title("Game Info");
}
