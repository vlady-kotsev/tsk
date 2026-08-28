use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, List, StatefulWidget},
};

use crate::models::BoardState;

pub struct Board<'a> {
    block: Block<'a>,
    style: Style,
    _layout: Layout,
}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        let block = Block::bordered().border_type(BorderType::Thick);
        let layout = Layout::vertical([Constraint::Fill(1)]);

        Self {
            block,
            style: Style::default().fg(Color::Magenta).bold(),
            _layout: layout,
        }
    }
}

impl<'a> StatefulWidget for Board<'a> {
    type State = BoardState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let tasks = state.tasks.iter().map(|task| task.content());
        let board_title = state.title.clone();
        let list = List::new(tasks)
            .style(Style::default().fg(Color::Cyan))
            .block(self.block.title(board_title).style(self.style))
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("#");
        list.render(area, buf, &mut state.list_state);
    }
}
