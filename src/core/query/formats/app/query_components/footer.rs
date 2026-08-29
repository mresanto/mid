use std::{collections::HashMap, time::Duration};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::core::{
    databases::adapters::database_type::DbValue,
    query::{TableCommand, formats::app::keybinds_events::KeybindEvents},
};

pub(crate) struct Footer<'a> {
    command: &'a TableCommand,
    duration: Duration,
    items: &'a [HashMap<String, DbValue>],
    filtered_count: usize,
    select_mode: bool,
    selected_count: usize,
}

impl<'a> Footer<'a> {
    pub(crate) const HEIGHT: u16 = 4;

    pub(crate) fn new(
        command: &'a TableCommand,
        duration: Duration,
        items: &'a [HashMap<String, DbValue>],
        filtered_count: usize,
        select_mode: bool,
        selected_count: usize,
    ) -> Self {
        Self {
            command,
            duration,
            items,
            filtered_count,
            select_mode,
            selected_count,
        }
    }

    fn commands(&self) -> Line<'static> {
        Line::from(
            KeybindEvents::footer_events(self.command, self.select_mode)
                .iter()
                .flat_map(|event| {
                    [
                        event.parse_to_command().yellow(),
                        format!(" {}  ", event.footer_label(self.select_mode)).into(),
                    ]
                })
                .collect::<Vec<_>>(),
        )
        .dark_gray()
    }
}

impl Widget for Footer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, duration_area, total_area, commands_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        Paragraph::new(Line::from(vec![
            "Duration: ".dark_gray(),
            format!("{:?}", self.duration).blue(),
        ]))
        .alignment(HorizontalAlignment::Left)
        .render(duration_area, buf);

        let mut lines = Line::from(vec![
            "Total Items: ".dark_gray(),
            self.items.len().to_string().blue(),
        ]);

        if self.filtered_count != self.items.len() {
            lines.extend(Line::from(vec![
                " (filtered: ".dark_gray(),
                self.filtered_count.to_string().blue(),
                ")".dark_gray(),
            ]));
        }

        if self.select_mode {
            lines.extend(Line::from(vec![
                " Selected: ".dark_gray(),
                self.selected_count.to_string().blue(),
            ]));
        }

        Paragraph::new(lines)
            .alignment(HorizontalAlignment::Left)
            .render(total_area, buf);

        Paragraph::new(self.commands())
            .alignment(HorizontalAlignment::Center)
            .render(commands_area, buf);
    }
}
