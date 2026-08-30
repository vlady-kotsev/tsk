use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskState {
    id: Uuid,
    content: String,
}

impl TaskState {
    #[must_use]
    pub fn new(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
        }
    }

    #[must_use]
    pub fn from_parts(id: Uuid, content: String) -> Self {
        Self { id, content }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}
