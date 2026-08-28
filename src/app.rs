use color_eyre::eyre::Result;
use ratatui::Terminal;
use ratatui::backend::Backend;

use crate::event::EventHandler;
use crate::message::{Direction, Message};
use crate::models::{AppModelState, RunningState, ScreenState};
use crate::view;
use std::cell::RefCell;

use std::sync::mpsc::{SyncSender, channel, sync_channel};
use std::time;
use std::{io, thread};

pub struct App {
    model: AppModelState,
    done_sender: Option<SyncSender<()>>,
}

impl App {
    pub fn new() -> Self {
        let model = AppModelState::new();

        Self {
            model,
            done_sender: None,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend<Error = io::Error>>) -> Result<()> {
        let (done_sender, done_receiver) = sync_channel::<()>(1);
        let (msg_sender, msg_receiver) = channel::<Message>();

        self.done_sender.replace(done_sender);

        let mut event_handler = EventHandler::new(msg_sender, done_receiver);

        let event_thread_handle = thread::spawn(move || {
            event_handler
                .listen()
                .expect("event handler failed to listen");
        });

        while self.model.running_state() != RunningState::Done {
            // Render the current view
            terminal.draw(|f| view::view(&mut self.model, f))?;

            // Handle events and map to a Message
            let mut current_msg = msg_receiver
                .recv_timeout(time::Duration::from_millis(250))
                .ok();

            // Process updates as long as they return a non-None message
            while current_msg.is_some() {
                current_msg = self.update(current_msg.unwrap());
            }
        }
        event_thread_handle.join().unwrap();
        Ok(())
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Up => {
                match self.model.screen_state() {
                    ScreenState::AllBoards => {
                        self.model.list_state.select_previous();
                    }
                    ScreenState::Board(board_index) => {
                        self.model
                            .boards
                            .get_mut(board_index)
                            .unwrap()
                            .list_state
                            .select_previous();
                    }
                    _ => {}
                };
                return Some(Message::Input('k'));
            }
            Message::Down => {
                match self.model.screen_state() {
                    ScreenState::AllBoards => {
                        self.model.list_state.select_next();
                    }
                    ScreenState::Board(board_index) => {
                        self.model
                            .get_board_at(board_index)?
                            .list_state
                            .select_next();
                    }
                    _ => {}
                };
                return Some(Message::Input('j'));
            }
            Message::Direction(direction) => {
                self.model.transition_state(direction);

                match direction {
                    Direction::Right => return Some(Message::Input('l')),
                    Direction::Left => return Some(Message::Input('h')),
                };
            }
            Message::Create => {
                if self.model.is_inputing() {
                    return Some(Message::Input('n'));
                } else {
                    self.model.set_is_inputing(true);
                    return None;
                }
            }
            Message::Quit => {
                if !self.model.is_inputing() {
                    self.model.set_running_state(RunningState::Done);
                    self.exit_event_loop();
                    return None;
                }
                return Some(Message::Input('q'));
            }
            Message::Input(c) => {
                if self.model.is_inputing() {
                    self.model.text_input_state.insert(c as u8);
                }

                return None;
            }
            Message::InputDelete => {
                if self.model.is_inputing() {
                    self.model.text_input_state.delete();
                }
                return None;
            }
            Message::InputMove(direction) => {
                match direction {
                    Direction::Right => self.model.text_input_state.move_right(),
                    Direction::Left => self.model.text_input_state.move_left(),
                };
                return None;
            }
            Message::InputDone => {
                let text = RefCell::new(self.model.text_input_state.take_text());
                match self.model.screen_state() {
                    ScreenState::Board(board_index) => {
                        self.model
                            .boards
                            .get_mut(board_index)
                            .unwrap()
                            .create_task(text.borrow_mut().to_string());
                    }
                    ScreenState::AllBoards => {
                        self.model.create_board(text.borrow_mut().to_string());
                    }
                    _ => return None,
                };
                self.model.set_is_inputing(false);
                return None;
            }
        };
    }

    pub fn exit_event_loop(&self) {
        if let Some(done_chan) = &self.done_sender {
            done_chan.send(()).expect("coundn't exit event loop");
        }
    }
}
