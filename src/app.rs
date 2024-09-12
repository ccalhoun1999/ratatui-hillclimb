use ratatui::{
    style::{Color, Stylize},
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Map, MapResolution},
        Block, Widget,
    },
};

pub struct App {
    pub marker: Marker,
    pub x: f64,
    pub y: f64,
}

impl App {
    pub fn new() -> App {
        App {
            marker: Marker::default(),
            x: 0.0,
            y: 0.0,
        }
    }
}
