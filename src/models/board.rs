use crate::models::TaskState;
use ratatui::widgets::ListState;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BoardState {
    id: Uuid,
    title: String,
    pub tasks: Vec<TaskState>,
    pub list_state: ListState,
}

impl BoardState {
    #[must_use]
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            tasks: vec![],
            list_state: ListState::default(),
        }
    }

    #[must_use]
    pub fn from_parts(id: Uuid, title: String, tasks: Vec<TaskState>) -> Self {
        Self {
            id,
            title,
            tasks,
            list_state: ListState::default(),
        }
    }

    #[must_use]
    pub fn id(&self) -> Uuid {
        self.id
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn create_task(&mut self, task_content: String) -> Option<TaskState> {
        if task_content.is_empty() {
            return None;
        }
        let task = TaskState::new(task_content);
        self.tasks.push(task.clone());
        Some(task)
    }

    pub fn remove_task(&mut self, task_index: usize) {
        self.tasks.remove(task_index);
    }

    pub fn get_task_at(&mut self, task_index: usize) -> Option<&mut TaskState> {
        if task_index >= self.tasks.len() {
            return None;
        }

        self.tasks.get_mut(task_index)
    }

    pub fn swap_tasks(&mut self, first_task_index: usize, second_task_index: usize) {
        if !(0..self.tasks.len()).contains(&first_task_index)
            || !(0..self.tasks.len()).contains(&second_task_index)
        {
            return;
        }

        self.tasks.swap(first_task_index, second_task_index);
    }
}
