use ratatui::widgets::ListState;

use crate::models::ItemState;

#[derive(Debug)]
pub struct BoardState {
    pub title: String,
    pub items: Vec<ItemState>,
    pub list_state: ListState,
}

impl BoardState {
    pub fn new(title: String) -> Self {
        Self {
            title,
            items: vec![],
            list_state: ListState::default(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn create_task(&mut self, task_content: String) {
        let index = self.items.len();
        let item = ItemState::new(index, task_content);
        self.items.push(item);
    }
}
