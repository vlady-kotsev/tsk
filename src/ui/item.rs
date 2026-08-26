use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::models::ItemState;

pub struct Item<'a> {
    block: Block<'a>,
    style: Style,
}

impl<'a> Item<'a> {
    pub fn new() -> Self {
        let block = Block::bordered().border_type(BorderType::Thick);

        Self {
            block,
            style: Style::default().fg(Color::Red).bold(),
        }
    }
}

impl<'a> StatefulWidget for Item<'a> {
    type State = ItemState;

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
