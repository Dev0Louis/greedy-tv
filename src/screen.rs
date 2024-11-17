use std::sync::Mutex;
use ratatui::Frame;
use ratatui::text::Line;
use zeroconf::ServiceDiscovery;

pub trait Screen {
    fn draw(&self, frame: &mut Vec<Line>, seconds_since_start: u64);
    fn on_key(&mut self);
}

pub struct ListScreen {
    pub(crate) discoveries: Mutex<Vec<ServiceDiscovery>>,
    pub(crate) index: Mutex<usize>
}

impl Screen for ListScreen {
    fn draw(&self, lines: &mut Vec<Line>, seconds_since_start: u64) {
        lines.push(Line::from("THIS WORKS!"))
    }

    fn on_key(&mut self) {
        todo!()
    }
}