use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::ui::theme::{WIN95_BLUE, title_bar_style};

#[derive(Debug, Default)]
pub struct TextInputState {
    cursor: u16,
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
        if let Some(ref text) = self.text
            && self.cursor as usize + 1 < text.len()
        {
            self.cursor += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.text = Some(vec![]);
        self.cursor = 0;
    }

    pub fn insert(&mut self, input: u8) {
        if let Some(ref mut text) = self.text {
            text.insert(self.cursor as usize, input);
            self.cursor += 1;
        }
    }

    pub fn delete(&mut self) {
        if let Some(ref mut text) = self.text
            && self.cursor > 0
        {
            text.remove(self.cursor as usize - 1);
            self.cursor -= 1;
        }
    }

    pub fn take_text(&mut self) -> String {
        self.cursor = 0;
        String::from_utf8(self.text.replace(vec![]).unwrap_or_default()).unwrap_or_default()
    }
}

pub struct TextInput<'a> {
    title: &'a str,
}

impl<'a> TextInput<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title }
    }
}

impl StatefulWidget for TextInput<'_> {
    type State = TextInputState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let block = Block::bordered()
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(WIN95_BLUE))
            .title(format!(" {} ", self.title))
            .title_style(title_bar_style());

        Paragraph::new(
            String::from_utf8(state.text.clone().unwrap_or_default()).expect("Invalid input"),
        )
        .style(Style::default().fg(Color::Magenta).bold())
        .block(block)
        .render(area, buf);
    }
}
