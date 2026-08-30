use crate::{
    models::{AppModelState, ScreenState},
    ui::{AppModel, Board, Task, TextInput, theme::centered_dialog},
};
use color_eyre::{Result, eyre::OptionExt};
use ratatui::{Frame, widgets::Clear};

pub fn view(model: &mut AppModelState, frame: &mut Frame) -> Result<()> {
    let app = AppModel::new("TSK");
    match model.screen_state() {
        ScreenState::AllBoards => {
            frame.render_stateful_widget(app, frame.area(), model);
        }
        ScreenState::Board(board_index) => {
            let board = Board::new();
            let board_state = model
                .get_board_at(board_index)
                .ok_or_eyre("board not found")?;
            frame.render_stateful_widget(board, frame.area(), board_state);
        }
        ScreenState::Task(board_index, task_index) => {
            let task = Task::new();
            let task_state = model
                .get_board_at(board_index)
                .ok_or_eyre("board not found")?
                .get_task_at(task_index)
                .ok_or_eyre("task not found")?;

            frame.render_stateful_widget(task, frame.area(), task_state);
        }
    }

    if model.is_inputing() {
        let title = match model.screen_state() {
            ScreenState::Board(_) => "New Task",
            _ => "New Board",
        };
        let dialog = centered_dialog(50, 3, frame.area());

        frame.render_widget(Clear, dialog);
        let text_input = TextInput::new(title);
        frame.render_stateful_widget(text_input, dialog, &mut model.text_input_state);

        frame.set_cursor_position((dialog.x + model.text_input_state.cursor() + 1, dialog.y + 1));
    }
    Ok(())
}
