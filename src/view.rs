use ratatui::Frame;

use crate::{
    models::AppModelState,
    ui::{AppModel, Board, Item},
};

pub fn view(model: &mut AppModelState, frame: &mut Frame) {
    // let board_title = if let Some(model) = model.get_selected_board() {
    //     model.title.clone()
    // } else {
    //     "No boards".to_string()
    // };
    // frame.render_widget(Paragraph::new(board_title), frame.area());
    let app = AppModel::new("TSK");

    match model.screen_state() {
        crate::models::ScreenState::AllBoards => {
            frame.render_stateful_widget(app, frame.area(), model);
        }
        crate::models::ScreenState::Board(board_index) => {
            let board = Board::new();
            let board_state = model.boards.get_mut(board_index).unwrap();
            frame.render_stateful_widget(board, frame.area(), board_state);
        }
        crate::models::ScreenState::Task(board_index, task_index) => {
            let item = Item::new();
            let item_state = model
                .boards
                .get_mut(board_index)
                .unwrap()
                .items
                .get_mut(task_index)
                .unwrap();

            frame.render_stateful_widget(item, frame.area(), item_state);
        }
    }
}
