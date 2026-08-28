use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::models::TaskState;

pub struct Task<'a> {
    block: Block<'a>,
    style: Style,
}

impl<'a> Task<'a> {
    pub fn new() -> Self {
        let block = Block::bordered().border_type(BorderType::Thick);

        Self {
            block,
            style: Style::default().fg(Color::Red).bold(),
        }
    }
}

impl<'a> StatefulWidget for Task<'a> {
    type State = TaskState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Paragraph::new(state.content())
            .block(self.block)
            .style(self.style)
            .render(area, buf);
    }
}
