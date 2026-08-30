use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::{
    models::TaskState,
    ui::theme::{WIN95_BLUE, centered_rect, footer_style, header_body_footer, title_bar_style},
};

pub struct Task;

impl Task {
    pub fn new() -> Self {
        Self
    }
}

impl StatefulWidget for Task {
    type State = TaskState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let (header, body, footer) = header_body_footer(area);

        Paragraph::new(" ▧ Task ")
            .style(title_bar_style())
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(WIN95_BLUE)),
            )
            .render(header, buf);

        let body = centered_rect(50, 50, body);
        Paragraph::new(state.content())
            .style(Style::default().fg(Color::Cyan).bold())
            .block(
                Block::bordered()
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::Magenta))
                    .title(" content ")
                    .title_style(Style::default().fg(Color::Magenta).bold()),
            )
            .render(body, buf);

        Paragraph::new(" h back   q quit ")
            .style(footer_style())
            .render(footer, buf);
    }
}
