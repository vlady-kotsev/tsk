use ratatui::widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget};

#[derive(Debug)]
pub struct TextInputState {
    cursor: usize,
    text: Option<Vec<u8>>,
}

impl TextInputState {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            text: Some(vec![]),
        }
    }

    pub fn cursor(&self) -> u16 {
        self.cursor as u16
    }

    pub fn move_right(&mut self) {
        if let Some(ref text) = self.text {
            if self.cursor + 1 < text.len() {
                self.cursor += 1;
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    pub fn insert(&mut self, input: u8) {
        if let Some(ref mut text) = self.text {
            text.insert(self.cursor, input);
            self.cursor += 1;
        }
    }

    pub fn delete(&mut self) {
        if let Some(ref mut text) = self.text {
            if self.cursor > 0 {
                text.remove(self.cursor - 1);
                self.cursor -= 1;
            }
        }
    }

    pub fn take_text(&mut self) -> String {
        self.cursor = 0;
        String::from_utf8(self.text.replace(vec![]).unwrap_or_default()).unwrap_or_default()
    }
}

pub struct TextInput<'a> {
    pub block: Block<'a>,
}

impl<'a> TextInput<'a> {
    pub fn new() -> Self {
        let block = Block::bordered().border_type(BorderType::Thick);
        Self { block }
    }
}

impl<'a> StatefulWidget for TextInput<'a> {
    type State = TextInputState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Paragraph::new(
            String::from_utf8(state.text.clone().unwrap_or_default()).expect("Invalid input"),
        )
        .block(self.block)
        // .style(self.style)
        .render(area, buf);
    }
}
