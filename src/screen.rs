use crate::{get_discoveries, get_discoveries_index};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Color, Span, Style};
use ratatui::style::{Stylize};
use ratatui::text::Line;
use std::ops::Add;
use std::process::exit;
use zeroconf::txt_record::TTxtRecord;
use zeroconf::ServiceDiscovery;

pub trait Screen {
    fn draw<'a>(&'a self, lines: &mut Vec<Line<'a>>, seconds_since_start: u64);
    fn on_key(&self, key_event: KeyEvent) -> Option<Box<dyn Screen>>;
}

pub struct ViewDiscoveryScreen {
    pub(crate) discovery: ServiceDiscovery
}

impl ViewDiscoveryScreen {
    fn style_span(string: String, alternative: bool) -> Span<'static> {
        if alternative {
            Span::from(string).style(Style::from(Color::White).bold())
        } else {
            Span::from(string).style(Style::from(Color::White).not_bold())
        }
    }
}

impl Screen for ViewDiscoveryScreen {
    fn draw<'a>(&'a self, lines: &mut Vec<Line<'a>>, _seconds_since_start: u64) {
        let name = self.discovery.name().clone();
        let hostname = self.discovery.host_name().clone().add(":").add(&self.discovery.port().to_string());
        let ip = self.discovery.domain().clone().add(":").add(&self.discovery.port().to_string());
        let mut grey = false;
        lines.push(Line::from("Name:").style(Style::from(Color::Blue).bg(Color::Black)));
        lines.push(Line::from(Self::style_span(name, grey))); grey = !grey;
        lines.push(Line::from("Hostname:").style(Style::from(Color::Blue).bg(Color::Black)));
        lines.push(Line::from(Self::style_span(hostname, grey))); grey = !grey;
        lines.push(Line::from("IP + Port:").style(Style::from(Color::Blue).bg(Color::Black)));
        lines.push(Line::from(Self::style_span(ip, grey))); grey = !grey;

        lines.push(Line::from("Service:").style(Style::from(Color::Blue).bg(Color::Black)));
        
        let service = self.discovery.service_type();
        lines.push(Line::from(Self::style_span(format!("[{},{}]", service.name(), service.protocol()), grey))); grey = !grey;
        lines.push(Line::from("Service Subtype:").style(Style::from(Color::Blue).bg(Color::Black)));
        
        
        let sub_types = service.sub_types();
        if sub_types.is_empty() {
            lines.push(Line::from(Self::style_span("No sub types.".to_string(), grey))); grey = !grey;
        } else {
            sub_types.iter().for_each(|t| lines.push(Line::from(t.clone())));
        }
        
        match self.discovery.txt() {
            None => {}
            Some(txt) => {
                lines.push(Line::from("Supplied Txt:").style(Style::from(Color::Blue).bg(Color::Black)));
                
                txt.iter().for_each(|t| {
                    lines.push(Line::from(format!("{} | {}", t.0, t.1)));
                })
            }
        };
  
    }
    
    fn on_key(&self, key_event: KeyEvent) -> Option<Box<dyn Screen>> {
        if key_event.code == KeyCode::Esc {
            Some(Box::from(ListScreen {}))
        } else {
            None
        }
    }
}


pub struct ListScreen {
    
}

impl Screen for ListScreen {
    fn draw(&self, lines: &mut Vec<Line>, _seconds_since_start: u64) {
        get_discoveries().lock().unwrap().iter().enumerate().for_each(|(index, service)| {
            let selection_indicator = get_symbol_for_index(index);

            let line: Span = {

                let span = Span::from(
                    format!(" {}, Hostname: {}, IP: {}, Port: {}",
                            service.name(),
                            service.host_name(),
                            service.address(),
                            service.port()
                    )
                );
                if index % 2 == 0 {
                    span.style(Style::from(Color::Cyan))
                } else {
                    span.style(Style::from(Color::LightCyan))
                }
            };

            lines.push(
                Line::from(vec![selection_indicator, line])
            );
        });
    }

    fn on_key(&self, key_event: KeyEvent) -> Option<Box<dyn 'static + Screen>> {
        if key_event.code == KeyCode::Up {
            let discoveries_index = &mut get_discoveries_index();
            let mut lock = discoveries_index.lock().unwrap();
            if *lock != 0 {
                (*lock) -= 1;
            }
        } else if key_event.code == KeyCode::Down {
            let discoveries_index = &mut get_discoveries_index();
            let mut lock = discoveries_index.lock().unwrap();
            if is_in_discoveries_range((*lock).clone() + 1) {
                (*lock) += 1;
            }
        } else if key_event.code == KeyCode::Enter {
            let discoveries_index = &mut get_discoveries_index();
            let index = *discoveries_index.lock().unwrap();
            let discoveries = get_discoveries().lock().unwrap();
            if !discoveries.is_empty() {
                let out: ServiceDiscovery = discoveries.get(index).unwrap().clone();
                return Some(Box::new(
                    ViewDiscoveryScreen {
                        discovery: out
                    }));
            } else { 
                return None;
            }
        } else if key_event.code == KeyCode::Esc {  
            exit(0);
        }
        None
    }
}

fn get_symbol_for_index(index: usize) -> Span<'static> {
    if is_selected(&index) {
        Span::from("[x]").style(Style::from(Color::White))
    } else {
        Span::from("[ ]").style(Style::from(Color::Gray))
    }
}

fn is_selected(index: &usize) -> bool {
    index.eq(&(*get_discoveries_index().lock().unwrap() as usize))
}

fn is_in_discoveries_range(index: usize) -> bool {
    let discoveries = &*get_discoveries().lock().unwrap();
    discoveries.len() > index
}