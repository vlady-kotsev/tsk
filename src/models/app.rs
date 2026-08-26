use ratatui::widgets::ListState;

use crate::{message::ChangeScreenDirection, models::BoardState};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ScreenState {
    #[default]
    AllBoards,
    Board(usize),
    Task(usize, usize),
}

#[derive(Debug, Default)]
pub struct AppModelState {
    running_state: RunningState,
    screen_state: ScreenState,
    pub boards: Vec<BoardState>,
    pub list_state: ListState,
}

impl AppModelState {
    pub fn create_board(&mut self, title: String) {
        self.boards.push(BoardState::new(title));
    }

    // pub fn boards(&self) -> &[Board] {
    //     &self.boards
    // }

    pub fn running_state(&self) -> RunningState {
        self.running_state
    }

    pub fn set_running_state(&mut self, new_state: RunningState) {
        self.running_state = new_state;
    }

    pub fn screen_state(&self) -> ScreenState {
        self.screen_state
    }

    // pub fn get_selected_board(&self) -> Option<&Board> {
    //     if self.list_state.selected().is_none() {
    //         return None;
    //     }
    //     Some(&self.boards[self.list_state.selected().unwrap() as usize])
    // }

    pub fn transition_state(&mut self, direction: ChangeScreenDirection) {
        match self.screen_state {
            ScreenState::AllBoards => {
                if direction == ChangeScreenDirection::In {
                    if let Some(board_index) = self.list_state.selected() {
                        self.screen_state = ScreenState::Board(board_index);
                    }
                }
            }
            ScreenState::Board(board_index) => match direction {
                ChangeScreenDirection::In => {
                    if let Some(task_index) =
                        self.boards.get(board_index).unwrap().list_state.selected()
                    {
                        self.screen_state = ScreenState::Task(board_index, task_index);
                    }
                }
                ChangeScreenDirection::Out => {
                    self.screen_state = ScreenState::AllBoards;
                }
            },
            ScreenState::Task(board_index, _) => {
                if direction == ChangeScreenDirection::Out {
                    self.screen_state = ScreenState::Board(board_index);
                }
            }
        }
    }
}
