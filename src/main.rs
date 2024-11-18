#[macro_use]
extern crate log;
mod client;
mod screen;

use crate::screen::{ListScreen, Screen};
use crossterm::event;
use crossterm::event::Event;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::Frame;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use zeroconf::ServiceDiscovery;

fn main() {
    thread::spawn(move || {
        client::start().expect("Client has boomed.");
    });
    
    let now = Instant::now();
    let screen: Box<Mutex<Box<dyn Screen>>> = Box::new(Mutex::new(Box::new(ListScreen {})));
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {draw(frame, now.elapsed().as_secs(), &screen)}).expect("failed to draw frame");
        if event::poll(Duration::from_millis(100)).unwrap() {
            let event = event::read().expect("failed to read event");
            match event {
                Event::Key(key_event) => {
                    let mut screen = screen.lock().unwrap();
                    match screen.on_key(key_event) {
                        Some(new_screen) => {
                            *screen = new_screen;
                        }
                        None => {}
                    };
                }
                _ => {}
            }
        }
    }
    //ratatui::restore();
}

fn draw(frame: &mut Frame, seconds: u64, screen: &Mutex<Box<dyn Screen>>) {
    let mut lines: Vec<Line> = Vec::new();
    let symbol = {
        if seconds % 2 == 0 {
            "Greedy TV 🍎"
        } else {
            "Greedy TV 💰"
        }
    };

    lines.push(Line::from(symbol).style(Style::from(Color::Green)));

    let screen = screen.lock().unwrap();
    screen.draw(&mut lines, seconds);
    
    let text = Text::from(lines);
    frame.render_widget(text, frame.area());
}



pub fn get_discoveries() -> &'static Mutex<Vec<ServiceDiscovery>> {
    static DISCOVERIES: Mutex<Vec<ServiceDiscovery>> = Mutex::new(Vec::<ServiceDiscovery>::new());
    &DISCOVERIES
}

pub fn get_discoveries_index() -> &'static Mutex<usize> {
    static INDEX: Mutex<usize> = Mutex::new(0);
    &INDEX
}