use color_eyre::eyre::Result;
use ratatui::Terminal;
use ratatui::backend::Backend;

use crate::event::EventHandler;
use crate::message::Message;
use crate::models::{AppModelState, RunningState, ScreenState};
use crate::view;
use std::time;
use std::{io, sync::mpsc, thread};

pub struct App {
    model: AppModelState,
}

impl App {
    pub fn new() -> Self {
        let model = AppModelState::default();

        Self { model }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend<Error = io::Error>>) -> Result<()> {
        let (sender, receiver) = mpsc::channel::<Message>();
        let mut event_handler = EventHandler::new(sender);

        let event_thread_handle = thread::spawn(move || {
            event_handler.listen().expect("Event handler failed");
        });

        while self.model.running_state() != RunningState::Done {
            // Render the current view
            terminal.draw(|f| view::view(&mut self.model, f))?;

            // Handle events and map to a Message
            let mut current_msg = receiver.recv_timeout(time::Duration::from_millis(250)).ok();

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
            Message::Up => match self.model.screen_state() {
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
            },
            Message::Down => match self.model.screen_state() {
                ScreenState::AllBoards => {
                    self.model.list_state.select_next();
                }
                ScreenState::Board(board_index) => {
                    self.model
                        .boards
                        .get_mut(board_index)
                        .unwrap()
                        .list_state
                        .select_next();
                }
                _ => {}
            },
            Message::ChangeScreen(direction) => self.model.transition_state(direction),
            Message::Create => match self.model.screen_state() {
                ScreenState::AllBoards => {
                    self.model.create_board("New board".to_string());
                }
                ScreenState::Board(board_index) => {
                    self.model
                        .boards
                        .get_mut(board_index)
                        .unwrap()
                        .create_task("My new task hehe".to_string());
                }
                _ => {}
            },
            Message::Quit => {
                self.model.set_running_state(RunningState::Done);
            }
        };
        None
    }
}
