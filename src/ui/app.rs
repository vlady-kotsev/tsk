use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, List, Paragraph, StatefulWidget, Widget},
};

use crate::{
    models::{AppModelState, BoardState},
    ui::theme::{WIN95_BLUE, footer_style, header_body_footer, selection_style, title_bar_style},
};

pub struct AppModel<'a> {
    app_name: &'a str,
}

impl<'a> AppModel<'a> {
    pub fn new(app_name: &'a str) -> Self {
        Self { app_name }
    }
}

impl StatefulWidget for AppModel<'_> {
    type State = AppModelState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let (header, body, footer) = header_body_footer(area);

        Paragraph::new(format!(" ▧ {} — Boards ", self.app_name))
            .style(title_bar_style())
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(WIN95_BLUE)),
            )
            .render(header, buf);

        let items = state.boards.iter().map(BoardState::title);
        let list = List::new(items)
            .style(Style::default().fg(Color::Magenta))
            .block(
                Block::bordered()
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title(" boards ")
                    .title_style(Style::default().fg(Color::Cyan).bold()),
            )
            .highlight_style(selection_style())
            .highlight_symbol("▶ ");
        StatefulWidget::render(list, body, buf, &mut state.list_state);

        Paragraph::new(" j/k move   l open   n new board   d delete   p copy   q quit ")
            .style(footer_style())
            .render(footer, buf);
    }
}
