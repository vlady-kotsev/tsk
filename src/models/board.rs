use ratatui::widgets::ListState;

use crate::models::TaskState;

#[derive(Debug)]
pub struct BoardState {
    pub title: String,
    pub tasks: Vec<TaskState>,
    pub list_state: ListState,
}

impl BoardState {
    pub fn new(title: String) -> Self {
        Self {
            title,
            tasks: vec![],
            list_state: ListState::default(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn create_task(&mut self, task_content: String) {
        let index = self.tasks.len();
        let task = TaskState::new(index, task_content);
        self.tasks.push(task);
    }
}
