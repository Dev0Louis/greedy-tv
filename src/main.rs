#[macro_use]
extern crate log;
mod client;
mod server;
mod screen;

use std::cell::RefCell;
use clap::Parser;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::Frame;
use std::fmt::Debug;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use crossterm::terminal::EnterAlternateScreen;
use zeroconf::ServiceDiscovery;
use crate::screen::{ListScreen, Screen};

fn main() {
    thread::spawn(move || {
        client::start().expect("Client has boomed.");
    });

    thread::spawn(move || {
        server::start().expect("Server has boomed.");
    });

    let now = Instant::now();
    let screen: Box<Mutex<&ListScreen>> = Box::new(Mutex::new(get_main_screen()));
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {draw(frame, now.elapsed().as_secs(), &screen)}).expect("failed to draw frame");
        if event::poll(Duration::from_millis(100)).unwrap() {
            let event = event::read().expect("failed to read event");
            match event {
                Event::Key(key_event) => {
                    if (key_event.code == KeyCode::Up) {
                        let discoveries_index = &mut get_discoveries_index();
                        let mut lock = discoveries_index.lock().unwrap();
                        if (*lock != 0) {
                            (*lock) -= 1;
                        }
                    } else if (key_event.code == KeyCode::Down) {
                        let discoveries_index = &mut get_discoveries_index();
                        let mut lock = discoveries_index.lock().unwrap();
                        if (is_in_discoveries_range((*lock).clone() + 1)) {
                            (*lock) += 1;
                        }
                    } else if key_event.code == KeyCode::Enter {
                        let discoveries_index = &mut get_discoveries_index();
                        let index = *discoveries_index.lock().unwrap();
                        let out: &ServiceDiscovery = get_discoveries().lock().unwrap().get(index as usize).unwrap();

                    }
                }
                _ => {}
            }
        }

    }
    ratatui::restore();
}

fn draw(frame: &mut Frame, seconds: u64, screen: &Mutex<&ListScreen>) {
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



    get_discoveries().lock().unwrap().iter().enumerate().for_each(|(index, service)| {
        let selection_indicator = get_symbol_for_index(index);

        let line: Span = {
            let line1 = Span::from(
                format!(" {}, IP: {}, Port: {}",
                        service.name(),
                        service.address(),
                        service.port()
                )
            );
            if (index % 2 == 0) {
                line1.style(Style::from(Color::Cyan))
            } else {
                line1.style(Style::from(Color::LightCyan))
            }
        };

        lines.push(
            Line::from(vec![selection_indicator, line])
        );
    });

    let text = Text::from(lines);
    frame.render_widget(text, frame.area());
}

fn get_symbol_for_index(index: usize) -> Span<'static> {
    if (is_selected(&index)) {
        Span::from("[x]").style(Style::from(Color::White))
    } else {
        Span::from("[ ]").style(Style::from(Color::Gray))
    }
}

fn is_selected(index: &usize) -> bool {
    index.eq(&(*get_discoveries_index().lock().unwrap() as usize))
}

fn is_in_discoveries_range(index: u8) -> bool {
    let discoveries = &*get_discoveries().lock().unwrap();
    discoveries.len() > index as usize
}

pub fn get_discoveries() -> &'static Mutex<Vec<ServiceDiscovery>> {
    &get_main_screen().discoveries
}

pub fn get_discoveries_index() -> &'static Mutex<u8> {
    get_main_screen().index
}

pub fn set_discoveries_index(newIndex: usize) {
    get_main_screen().index = newIndex
}

pub fn get_main_screen() -> &'static mut ListScreen {
    static mut MAIN_SCREEN: ListScreen = ListScreen {
        discoveries: Mutex::new(Vec::<ServiceDiscovery>::new()),
        index: 0
    };
    &mut MAIN_SCREEN
}