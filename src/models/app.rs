use ratatui::widgets::ListState;

use crate::{message::Direction, models::BoardState, ui::TextInputState};

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

#[derive(Debug)]
pub struct AppModelState {
    running_state: RunningState,
    screen_state: ScreenState,
    pub boards: Vec<BoardState>,
    pub list_state: ListState,
    pub text_input_state: TextInputState,
    is_inputing: bool,
}

impl AppModelState {
    pub fn new() -> Self {
        Self {
            running_state: RunningState::Running,
            screen_state: ScreenState::AllBoards,
            boards: vec![],
            list_state: ListState::default(),
            text_input_state: TextInputState::new(),
            is_inputing: false,
        }
    }
    pub fn create_board(&mut self, title: String) {
        self.boards.push(BoardState::new(title));
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

    pub fn set_screen_state(&mut self, state: ScreenState) {
        self.screen_state = state;
    }

    pub fn is_inputing(&self) -> bool {
        self.is_inputing
    }

    pub fn set_is_inputing(&mut self, is_inputing: bool) {
        self.is_inputing = is_inputing;
    }

    pub fn transition_state(&mut self, direction: Direction) {
        match self.screen_state {
            ScreenState::AllBoards => {
                if direction == Direction::Right {
                    if let Some(board_index) = self.list_state.selected() {
                        self.screen_state = ScreenState::Board(board_index);
                    }
                }
            }
            ScreenState::Board(board_index) => match direction {
                Direction::Right => {
                    if let Some(task_index) =
                        self.boards.get(board_index).unwrap().list_state.selected()
                    {
                        self.screen_state = ScreenState::Task(board_index, task_index);
                    }
                }
                Direction::Left => {
                    self.screen_state = ScreenState::AllBoards;
                }
            },
            ScreenState::Task(board_index, _) => {
                if direction == Direction::Left {
                    self.screen_state = ScreenState::Board(board_index);
                }
            }
        }
    }

    pub fn get_board_at(&mut self, board_index: usize) -> Option<&mut BoardState> {
        if board_index >= self.boards.len() {
            return None;
        }
        Some(
            self.boards
                .get_mut(board_index)
                .expect("this should be here"),
        )
    }
}
