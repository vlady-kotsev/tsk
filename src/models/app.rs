use color_eyre::{Result, eyre::OptionExt};
use ratatui::widgets::ListState;

use crate::{message::DirectionX, models::BoardState, persistence, ui::TextInputState};

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
    pub text_input_state: TextInputState,
    is_inputing: bool,
}

impl AppModelState {
    pub fn load() -> Result<Self> {
        let boards = persistence::load_boards()?;
        Ok(Self {
            running_state: RunningState::Running,
            screen_state: ScreenState::AllBoards,
            boards,
            list_state: ListState::default(),
            text_input_state: TextInputState::new(),
            is_inputing: false,
        })
    }
    pub fn create_board(&mut self, title: String) -> Option<BoardState> {
        if title.is_empty() {
            return None;
        }
        let board = BoardState::new(title);
        self.boards.push(board.clone());
        Some(board)
    }

    pub fn remove_board(&mut self, board_index: usize) {
        self.boards.remove(board_index);
    }

    pub fn running_state(&self) -> RunningState {
        self.running_state
    }

    pub fn set_running_state(&mut self, new_state: RunningState) {
        self.running_state = new_state;
    }

    pub fn screen_state(&self) -> ScreenState {
        self.screen_state
    }

    pub fn is_inputing(&self) -> bool {
        self.is_inputing
    }

    pub fn set_is_inputing(&mut self, is_inputing: bool) {
        self.is_inputing = is_inputing;
    }

    pub fn transition_state(&mut self, direction: DirectionX) -> Result<()> {
        match self.screen_state {
            ScreenState::AllBoards => {
                if direction == DirectionX::Right
                    && let Some(board_index) = self.list_state.selected()
                {
                    self.screen_state = ScreenState::Board(board_index);
                }
            }
            ScreenState::Board(board_index) => match direction {
                DirectionX::Right => {
                    if let Some(task_index) = self
                        .get_board_at(board_index)
                        .ok_or_eyre("board not found")?
                        .list_state
                        .selected()
                    {
                        self.screen_state = ScreenState::Task(board_index, task_index);
                    }
                }
                DirectionX::Left => {
                    self.screen_state = ScreenState::AllBoards;
                }
            },
            ScreenState::Task(board_index, _) => {
                if direction == DirectionX::Left {
                    self.screen_state = ScreenState::Board(board_index);
                }
            }
        };
        Ok(())
    }

    pub fn get_board_at(&mut self, board_index: usize) -> Option<&mut BoardState> {
        if board_index >= self.boards.len() {
            return None;
        }

        self.boards.get_mut(board_index)
    }

    pub fn swap_boards(&mut self, first_board_index: usize, second_board_index: usize) {
        if !(0..self.boards.len()).contains(&first_board_index)
            || !(0..self.boards.len()).contains(&second_board_index)
        {
            return;
        }

        self.boards.swap(first_board_index, second_board_index);
    }
}
