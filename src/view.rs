use ratatui::Frame;

use crate::{
    models::{AppModelState, ScreenState},
    ui::{AppModel, Board, Task, TextInput},
};

pub fn view(model: &mut AppModelState, frame: &mut Frame) {
    // let board_title = if let Some(model) = model.get_selected_board() {
    //     model.title.clone()
    // } else {
    //     "No boards".to_string()
    // };
    // frame.render_widget(Paragraph::new(board_title), frame.area());
    let app = AppModel::new("TSK");
    if model.is_inputing() {
        let text_input = TextInput::new();
        frame.render_stateful_widget(text_input, frame.area(), &mut model.text_input_state);

        frame.set_cursor_position((
            frame.area().x + model.text_input_state.cursor() + 1,
            frame.area().y + 1,
        ));
    } else {
        match model.screen_state() {
            ScreenState::AllBoards => {
                frame.render_stateful_widget(app, frame.area(), model);
            }
            ScreenState::Board(board_index) => {
                let board = Board::new();
                let board_state = model.boards.get_mut(board_index).unwrap();
                frame.render_stateful_widget(board, frame.area(), board_state);
            }
            ScreenState::Task(board_index, task_index) => {
                let task = Task::new();
                let task_state = model
                    .boards
                    .get_mut(board_index)
                    .unwrap()
                    .tasks
                    .get_mut(task_index)
                    .unwrap();

                frame.render_stateful_widget(task, frame.area(), task_state);
            }
        }
    }
}
