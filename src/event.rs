use std::{
    sync::mpsc::{Receiver, TryRecvError},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::message::{DirectionX, DirectionY, Message};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::sync::mpsc::Sender;

struct EventHandler {
    event_channel: Sender<Message>,
    done_channel: Receiver<()>,
}

const TIMEOUT: Duration = Duration::from_millis(250);

impl EventHandler {
    fn new(event_channel: Sender<Message>, done_channel: Receiver<()>) -> Self {
        Self {
            event_channel,
            done_channel,
        }
    }

    fn listen(&mut self) -> Result<()> {
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
                && let Some(msg) = handle_key(key)
            {
                self.event_channel.send(msg)?;
            }
        }
    }
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::Down),
        KeyCode::Char('k') => Some(Message::Up),
        KeyCode::Char('q') => Some(Message::Quit('q')),
        KeyCode::Char('l') => Some(Message::Direction(DirectionX::Right)),
        KeyCode::Char('h') => Some(Message::Direction(DirectionX::Left)),
        KeyCode::Char('p') => Some(Message::ClipboardCopy),
        KeyCode::Char('c') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                Some(Message::Quit('c'))
            } else {
                Some(Message::Input('c'))
            }
        }
        KeyCode::Char('n') => Some(Message::Create),
        KeyCode::Char('d') => Some(Message::Delete),
        KeyCode::Char(other) => Some(Message::Input(other)),
        KeyCode::Backspace => Some(Message::InputDelete),
        KeyCode::Left => Some(Message::InputMove(DirectionX::Left)),
        KeyCode::Right => Some(Message::InputMove(DirectionX::Right)),
        KeyCode::Enter => Some(Message::InputDone),
        KeyCode::Esc => Some(Message::InputCancel),
        KeyCode::Up => Some(Message::MoveOrder(DirectionY::Up)),
        KeyCode::Down => Some(Message::MoveOrder(DirectionY::Down)),
        _ => None,
    }
}

pub fn run(msg_sender: Sender<Message>, done_receiver: Receiver<()>) -> JoinHandle<()> {
    let mut event_handler = EventHandler::new(msg_sender, done_receiver);
    thread::spawn(move || {
        event_handler
            .listen()
            .expect("event handler failed to listen");
    })
}
