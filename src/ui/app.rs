use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, List, StatefulWidget},
};

use crate::models::AppModelState;

pub struct AppModel<'a> {
    app_name: &'a str,
    style: Style,
    block: Block<'a>,
}

impl<'a> AppModel<'a> {
    pub fn new(app_name: &'a str) -> Self {
        let block = Block::bordered().border_type(BorderType::Thick);
        Self {
            app_name,
            style: Style::default().fg(Color::Cyan).bold(),
            block,
        }
    }
}

impl<'a> StatefulWidget for AppModel<'a> {
    type State = AppModelState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let items = state.boards.iter().map(|board| board.title());

        let list = List::new(items)
            .style(Style::default().fg(Color::Magenta))
            .block(self.block.title(self.app_name).style(self.style))
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("#");
        list.render(area, buf, &mut state.list_state);
    }
}
