use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Widget, Wrap},
};

pub(crate) struct Header<'a> {
    query: &'a str,
    query_expanded: bool,
    value: Option<String>,
    value_expanded: bool,
}

impl<'a> Header<'a> {
    pub(crate) fn new(
        query: &'a str,
        query_expanded: bool,
        value: Option<String>,
        value_expanded: bool,
    ) -> Self {
        Self {
            query,
            query_expanded,
            value,
            value_expanded,
        }
    }

    fn query_lines(&self) -> Vec<Line<'_>> {
        if self.query_expanded {
            let mut lines = vec![Line::from("Query:")];
            lines.extend(
                self.query
                    .lines()
                    .map(|line| Line::from(line.to_string().yellow())),
            );
            lines
        } else {
            vec![Line::from(vec![
                "Query [e to expand]: ".into(),
                self.query.replace(['\n', '\r'], " ").yellow(),
            ])]
        }
    }

    fn value_line(&self) -> Option<Line<'static>> {
        self.value
            .as_ref()
            .filter(|_| self.value_expanded)
            .map(|value| Line::from(vec!["Value: ".into(), value.clone().cyan()]))
    }

    pub(crate) fn height(&self, available_width: usize) -> u16 {
        let query_height = if self.query_expanded {
            self.query_lines()
                .iter()
                .map(|line| line.width().div_ceil(available_width).max(1))
                .sum::<usize>()
        } else {
            1
        };
        let value_height = usize::from(self.value_line().is_some());

        query_height
            .saturating_add(value_height)
            .min(u16::MAX as usize) as u16
    }
}

impl Widget for Header<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let query_lines = self.query_lines();
        let value_line = self.value_line();
        let value_height = u16::from(value_line.is_some());
        let [query_area, value_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(value_height)]).areas(area);

        let query = Paragraph::new(query_lines);
        if self.query_expanded {
            query.wrap(Wrap { trim: false }).render(query_area, buf);
        } else {
            query.render(query_area, buf);
        }

        if let Some(value_line) = value_line {
            Paragraph::new(value_line)
                .wrap(Wrap { trim: false })
                .render(value_area, buf);
        }
    }
}
