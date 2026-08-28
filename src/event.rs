use std::{
    sync::mpsc::{Receiver, TryRecvError},
    time::Duration,
};

use crate::message::{Direction, Message};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::sync::mpsc::Sender;

pub struct EventHandler {
    event_channel: Sender<Message>,
    done_channel: Receiver<()>,
}

const TIMEOUT: Duration = Duration::from_millis(250);

impl EventHandler {
    pub fn new(event_channel: Sender<Message>, done_channel: Receiver<()>) -> Self {
        Self {
            event_channel,
            done_channel,
        }
    }

    pub fn listen(&mut self) -> Result<()> {
        loop {
            if matches!(
                self.done_channel.try_recv(),
                Ok(()) | Err(TryRecvError::Disconnected)
            ) {
                return Ok(());
            }

            if event::poll(TIMEOUT)?
                && let Event::Key(key) = event::read()?
                && key.kind == event::KeyEventKind::Press
            {
                if let Some(msg) = handle_key(key) {
                    self.event_channel.send(msg)?;
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
        KeyCode::Char('l') => Some(Message::Direction(Direction::Right)),
        KeyCode::Char('h') => Some(Message::Direction(Direction::Left)),
        KeyCode::Char('c') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                Some(Message::Quit)
            } else {
                None
            }
        }
        KeyCode::Char('n') => Some(Message::Create),
        KeyCode::Char(other) => Some(Message::Input(other)),
        KeyCode::Backspace => Some(Message::InputDelete),
        KeyCode::Left => Some(Message::InputMove(Direction::Left)),
        KeyCode::Right => Some(Message::InputMove(Direction::Right)),
        KeyCode::Enter => Some(Message::InputDone),
        _ => None,
    }
}
