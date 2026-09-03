use color_eyre::eyre::Result;
use ratatui::Terminal;
use ratatui::backend::Backend;

use crate::clipboard::Clipboard;
use crate::event;
use crate::message::{DirectionX, DirectionY, Message};
use crate::models::{AppModelState, RunningState, ScreenState};
use crate::persistence::{self, PersistMessage};
use crate::view;
use std::cell::RefCell;

use std::io;
use std::sync::Arc;
use std::sync::mpsc::{Sender, SyncSender, channel, sync_channel};
use std::time;

pub struct App {
    model: AppModelState,
    done_sender: Option<SyncSender<()>>,
    persist_msg_sender: Option<Sender<PersistMessage>>,
    clipboard: Clipboard,
}

impl App {
    pub fn new() -> Result<Self> {
        let model = AppModelState::load()?;
        let clipboard = Clipboard::new()?;
        Ok(Self {
            model,
            done_sender: None,
            persist_msg_sender: None,
            clipboard,
        })
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend<Error = io::Error>>) -> Result<()> {
        let (done_sender, done_receiver) = sync_channel::<()>(1);
        let (msg_sender, msg_receiver) = channel::<Message>();
        let (persist_msg_sender, persist_msg_receiver) = channel::<PersistMessage>();

        self.persist_msg_sender.replace(persist_msg_sender);
        self.done_sender.replace(done_sender);

        let persistenc_thread_handle = persistence::run(persist_msg_receiver)?;

        let event_thread_handle = event::run(msg_sender, done_receiver);

        while self.model.running_state() != RunningState::Done {
            terminal.draw(|f| view::view(&mut self.model, f).expect("failed to draw view"))?;

            let mut current_msg = msg_receiver
                .recv_timeout(time::Duration::from_millis(250))
                .ok();

            while let Some(msg) = current_msg {
                current_msg = self.update(msg);
            }
        }
        event_thread_handle.join().unwrap();
        let _ = persistenc_thread_handle.join().unwrap();
        Ok(())
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        if self.model.is_inputing() {
            self.update_inputing(msg)
        } else {
            self.update_navigating(msg)
        }
    }

    fn update_inputing(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Up => Some(Message::Input('k')),
            Message::Down => Some(Message::Input('j')),
            Message::Direction(direction) => match direction {
                DirectionX::Right => Some(Message::Input('l')),
                DirectionX::Left => Some(Message::Input('h')),
            },
            Message::Create => Some(Message::Input('n')),
            Message::Quit(c) => Some(Message::Input(c)),
            Message::Delete => Some(Message::Input('d')),
            Message::MoveOrder(_) => None,
            Message::Input(c) => {
                self.model.text_input_state.insert(c as u8);
                None
            }
            Message::InputDelete => {
                self.model.text_input_state.delete();
                None
            }
            Message::InputMove(direction) => {
                match direction {
                    DirectionX::Right => self.model.text_input_state.move_right(),
                    DirectionX::Left => self.model.text_input_state.move_left(),
                };
                None
            }
            Message::InputDone => {
                let text = RefCell::new(self.model.text_input_state.take_text());
                match self.model.screen_state() {
                    ScreenState::Board(board_index) => {
                        let board = self.model.get_board_at(board_index)?;
                        let board_id = board.id();
                        let task = board.create_task(text.borrow_mut().to_string());

                        if let Some(task) = task
                            && let Some(sender) = self.persist_msg_sender.as_ref()
                        {
                            let _ =
                                sender.send(PersistMessage::CreateTask(board_id, Arc::new(task)));
                        }
                    }
                    ScreenState::AllBoards => {
                        let board = self.model.create_board(text.borrow_mut().to_string());
                        if let Some(board) = board
                            && let Some(sender) = self.persist_msg_sender.as_ref()
                        {
                            let _ = sender.send(PersistMessage::CreateBoard(Arc::new(board)));
                        }
                    }
                    _ => return None,
                };
                self.model.set_is_inputing(false);
                None
            }
            Message::InputCancel => {
                self.model.text_input_state.reset();
                self.model.set_is_inputing(false);
                None
            }
            Message::ClipboardCopy => Some(Message::Input('p')),
        }
    }

    fn update_navigating(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Up => {
                match self.model.screen_state() {
                    ScreenState::AllBoards => {
                        self.model.list_state.select_previous();
                    }
                    ScreenState::Board(board_index) => {
                        self.model
                            .get_board_at(board_index)?
                            .list_state
                            .select_previous();
                    }
                    _ => {}
                };
                Some(Message::Input('k'))
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
                Some(Message::Input('j'))
            }
            Message::Direction(direction) => {
                if self.model.transition_state(direction).is_err() {
                    return None;
                }
                match direction {
                    DirectionX::Right => Some(Message::Input('l')),
                    DirectionX::Left => Some(Message::Input('h')),
                }
            }
            Message::Create => {
                self.model.set_is_inputing(true);
                None
            }
            Message::Quit(c) => {
                self.model.set_running_state(RunningState::Done);
                self.exit_event_loop();
                Some(Message::Input(c))
            }
            Message::Input(_) => None,
            Message::InputDelete => None,
            Message::InputMove(direction) => {
                match direction {
                    DirectionX::Right => self.model.text_input_state.move_right(),
                    DirectionX::Left => self.model.text_input_state.move_left(),
                };
                None
            }
            Message::InputDone => None,
            Message::InputCancel => None,
            Message::Delete => {
                match self.model.screen_state() {
                    ScreenState::Board(board_index) => {
                        let board = self.model.get_board_at(board_index)?;
                        let task_index = board.list_state.selected()?;
                        let task_id = board.get_task_at(task_index)?.id();
                        board.remove_task(task_index);

                        if let Some(sender) = self.persist_msg_sender.as_ref() {
                            let _ = sender.send(PersistMessage::DeleteTask(task_id));
                        }
                    }
                    ScreenState::AllBoards => {
                        let board_index = self.model.list_state.selected()?;
                        if !self.model.get_board_at(board_index)?.tasks.is_empty() {
                            return None;
                        }
                        let board_id = self.model.get_board_at(board_index)?.id();
                        self.model.remove_board(board_index);

                        if let Some(sender) = self.persist_msg_sender.as_ref() {
                            let _ = sender.send(PersistMessage::DeleteBoard(board_id));
                        }
                    }
                    _ => return None,
                };
                None
            }
            Message::MoveOrder(direction_y) => {
                match self.model.screen_state() {
                    ScreenState::Board(board_index) => {
                        let board = self.model.get_board_at(board_index)?;
                        let task_index = board.list_state.selected()?;

                        match direction_y {
                            DirectionY::Up => {
                                board.swap_tasks(task_index, task_index.checked_sub_signed(1)?)
                            }
                            DirectionY::Down => {
                                board.swap_tasks(task_index, task_index.checked_add_signed(1)?)
                            }
                        }
                    }
                    ScreenState::AllBoards => {
                        let board_index = self.model.list_state.selected()?;

                        match direction_y {
                            DirectionY::Up => self
                                .model
                                .swap_boards(board_index, board_index.checked_sub_signed(1)?),

                            DirectionY::Down => self
                                .model
                                .swap_boards(board_index, board_index.checked_add_signed(1)?),
                        }
                    }
                    _ => return None,
                };
                None
            }
            Message::ClipboardCopy => {
                match self.model.screen_state() {
                    ScreenState::AllBoards => {
                        let board_index = self.model.list_state.selected()?;
                        let board = self.model.get_board_at(board_index)?;
                        self.clipboard.copy_to_clipboard(board.title()).ok()?;
                    }
                    ScreenState::Board(board_index) => {
                        let board = self.model.get_board_at(board_index)?;
                        let task_index = board.list_state.selected()?;
                        let task = board.get_task_at(task_index)?;
                        self.clipboard.copy_to_clipboard(task.content()).ok()?;
                    }
                    ScreenState::Task(board_index, task_index) => {
                        let board = self.model.get_board_at(board_index)?;
                        let task = board.get_task_at(task_index)?;
                        self.clipboard.copy_to_clipboard(task.content()).ok()?;
                    }
                };
                None
            }
        }
    }

    pub fn exit_event_loop(&self) {
        if let Some(done_chan) = &self.done_sender {
            done_chan.send(()).expect("coundn't exit event loop");
        }
        if let Some(sender) = &self.persist_msg_sender {
            let _ = sender.send(PersistMessage::QuitPersister);
        }
    }
}
