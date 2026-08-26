#[derive(Debug)]
pub struct ItemState {
    _index: usize,
    content: String,
}

impl ItemState {
    pub fn new(index: usize, content: String) -> Self {
        Self {
            _index: index,
            content,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}
