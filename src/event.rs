use std::time;

use crate::message::{ChangeScreenDirection, Message};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::sync::mpsc::Sender;

pub struct EventHandler {
    event_channel: Sender<Message>,
}

impl EventHandler {
    pub fn new(event_channel: Sender<Message>) -> Self {
        Self { event_channel }
    }

    pub fn listen(&mut self) -> Result<()> {
        loop {
            if event::poll(time::Duration::from_millis(250))?
                && let Event::Key(key) = event::read()?
                && key.kind == event::KeyEventKind::Press
            {
                if let Some(msg) = handle_key(key) {
                    self.event_channel.send(msg)?;
                    if msg == Message::Quit {
                        return Ok(());
                    }
                }
            }
        }
    }
}

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::Down),
        KeyCode::Char('k') => Some(Message::Up),
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('l') => Some(Message::ChangeScreen(ChangeScreenDirection::In)),
        KeyCode::Char('h') => Some(Message::ChangeScreen(ChangeScreenDirection::Out)),
        KeyCode::Char('c') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                Some(Message::Quit)
            } else {
                None
            }
        }
        KeyCode::Char('n') => Some(Message::Create),
        _ => None,
    }
}
