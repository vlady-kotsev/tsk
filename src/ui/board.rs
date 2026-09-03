use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, List, Paragraph, StatefulWidget, Widget},
};

use crate::{
    models::{BoardState, TaskState},
    ui::theme::{WIN95_BLUE, footer_style, header_body_footer, selection_style, title_bar_style},
};

pub struct Board;

impl Board {
    pub fn new() -> Self {
        Self
    }
}

impl StatefulWidget for Board {
    type State = BoardState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let (header, body, footer) = header_body_footer(area);

        Paragraph::new(format!(" ▧ {} ", state.title()))
            .style(title_bar_style())
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(WIN95_BLUE)),
            )
            .render(header, buf);

        let tasks = state.tasks.iter().map(TaskState::content);
        let list = List::new(tasks)
            .style(Style::default().fg(Color::Cyan))
            .block(
                Block::bordered()
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::Magenta))
                    .title(" tasks ")
                    .title_style(Style::default().fg(Color::Magenta).bold()),
            )
            .highlight_style(selection_style())
            .highlight_symbol("▶ ");
        StatefulWidget::render(list, body, buf, &mut state.list_state);

        Paragraph::new(" j/k move   l open   h back   n new task   d delete   p copy   q quit ")
            .style(footer_style())
            .render(footer, buf);
    }
}
