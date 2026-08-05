use std::{
    collections::{BTreeSet, HashMap},
    io,
    time::Duration,
};

use arboard::Clipboard;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Paragraph, Row, StatefulWidget, Table, TableState, Widget, Wrap},
};

use crate::core::{
    databases::application::{query::DbValue, tables::update::update_database_table},
    query::{TableCommand, TableEvent},
};

const COLUMN_WIDTH: u16 = 20;
const COLUMN_SPACING: u16 = 1;
const FOOTER_HEIGHT: u16 = 3;

#[derive(Default)]
pub struct App {
    items: Vec<HashMap<String, DbValue>>,
    command: TableCommand,
    query: String,
    query_expanded: bool,
    value_expanded: bool,
    exit: bool,
    table_state: TableState,
    column_offset: usize,
    selected_column: usize,
    clipboard: Option<Clipboard>,
    event: Option<TableEvent>,
    duration: Duration,
}

impl App {
    pub fn new(items: Vec<HashMap<String, DbValue>>, command: TableCommand, query: String) -> Self {
        let mut table_state = TableState::default();
        if !items.is_empty() {
            table_state.select_first();
        }

        Self {
            items,
            command,
            query,
            table_state,
            clipboard: Clipboard::new().ok(),
            ..Self::default()
        }
    }

    pub fn update_app_results(
        &mut self,
        items: Vec<HashMap<String, DbValue>>,
        query: String,
        duration: Duration,
    ) {
        self.items = items;
        self.query = query;
        self.duration = duration;
        self.exit = false;
        self.event = None;
        self.value_expanded = false;
        self.column_offset = 0;
        self.selected_column = 0;
        self.table_state = TableState::default();
        if !self.items.is_empty() {
            self.table_state.select_first();
        }
    }

    fn format_db_value(value: &DbValue) -> String {
        match value {
            DbValue::Null => "null".to_string(),
            DbValue::Text(value) => value.clone(),
            DbValue::TextArray(values) => {
                format!("{{{}}}", values.join(","))
            }
            DbValue::Numeric(value) => value.clone(),
            DbValue::Integer(value) => value.to_string(),
            DbValue::Float(value) => {
                if value.is_finite() {
                    value.to_string()
                } else {
                    "null".to_string()
                }
            }
            DbValue::Boolean(value) => value.to_string(),
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<Option<TableEvent>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(self.event.take())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Char('q') {
            self.exit();
            return;
        }

        match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => self.select_next_row(),
            KeyCode::Up | KeyCode::Char('k') => self.select_previous_row(),
            KeyCode::Right | KeyCode::Char('l') => self.select_next_column(),
            KeyCode::Left | KeyCode::Char('h') => self.select_previous_column(),
            KeyCode::Char('g') => self.select_first_row(),
            KeyCode::Char('G') => self.select_last_row(),
            KeyCode::Char('y') => self.yank_selected_row(),
            KeyCode::Char('u') => match self.command {
                TableCommand::ShowValue => self.update_selected_value(),
                TableCommand::ShowTables => {}
            },
            KeyCode::Char('e') => self.toggle_query_expanded(),
            KeyCode::Char('p') => self.edit_query(),
            KeyCode::Enter => match self.command {
                TableCommand::ShowTables => self.select_table(),
                TableCommand::ShowValue => self.toggle_value_expanded(),
            },
            _ => {}
        }
    }

    fn edit_query(&mut self) {
        self.event = Some(TableEvent::EditQuery(self.query.clone()));
        self.exit();
    }

    fn update_selected_value(&mut self) {
        let Some(selected_row) = self
            .table_state
            .selected()
            .and_then(|selected| self.items.get(selected))
        else {
            return;
        };
        let Some(column) = self.selected_column_name() else {
            return;
        };
        let Some(value) = selected_row.get(&column) else {
            return;
        };
        // let Some(id_column) = self.first_column_name() else {
        //     return;
        // };q
        let id_column = "Id";
        let Some(id) = selected_row.get(id_column) else {
            panic!("No id column found in selected row");
        };
        let Some(table) = Self::table_from_query(&self.query) else {
            return;
        };
        let Ok(update_query) = update_database_table(&table, &id_column, id, &column, value) else {
            return;
        };
        self.event = Some(TableEvent::UpdateValue(update_query));
        self.exit();
    }

    fn table_from_query(query: &str) -> Option<String> {
        let words = query.split_whitespace().collect::<Vec<_>>();
        let from = words
            .iter()
            .position(|word| word.eq_ignore_ascii_case("FROM"))?;
        words.get(from + 1).map(|table| {
            let table = table.trim_end_matches(|character| character == ';' || character == ',');
            if let Some(table) = table
                .strip_prefix('"')
                .and_then(|table| table.strip_suffix('"'))
            {
                table.replace("\"\"", "\"")
            } else if let Some(table) = table
                .strip_prefix('`')
                .and_then(|table| table.strip_suffix('`'))
            {
                table.replace("``", "`")
            } else {
                table.to_string()
            }
        })
    }

    fn selected_column_name(&self) -> Option<String> {
        self.items
            .iter()
            .flat_map(|row| row.keys().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .nth(self.selected_column)
    }

    // fn first_column_name(&self) -> Option<String> {
    //     self.items
    //         .iter()
    //         .flat_map(|row| row.keys().cloned())
    //         .collect::<BTreeSet<_>>()
    //         .into_iter()
    //         .next()
    // }

    fn select_table(&mut self) {
        let Some(table_name) = self
            .table_state
            .selected()
            .and_then(|selected| self.items.get(selected))
            .and_then(|row| row.get("table_name"))
            .map(Self::format_db_value)
        else {
            return;
        };

        self.event = Some(TableEvent::SelectTable(table_name));
        self.exit();
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn toggle_query_expanded(&mut self) {
        self.query_expanded = !self.query_expanded;
    }

    fn toggle_value_expanded(&mut self) {
        if self.selected_value().is_some() {
            self.value_expanded = !self.value_expanded;
        }
    }

    fn select_next_row(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let current_row = self.table_state.selected().unwrap_or(0);
        let last_row = self.items.len().saturating_sub(1);
        self.table_state
            .select(Some(current_row.saturating_add(1).min(last_row)));
    }

    fn select_previous_row(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let current_row = self.table_state.selected().unwrap_or(0);
        self.table_state.select(Some(current_row.saturating_sub(1)));
    }

    fn select_next_column(&mut self) {
        let last_column = self.column_count().saturating_sub(1);
        self.selected_column = self.selected_column.saturating_add(1).min(last_column);
    }

    fn select_previous_column(&mut self) {
        self.selected_column = self.selected_column.saturating_sub(1);
    }

    fn select_first_row(&mut self) {
        if !self.items.is_empty() {
            self.table_state.select_first();
        }
    }

    fn select_last_row(&mut self) {
        if !self.items.is_empty() {
            self.table_state.select_last();
        }
    }

    fn yank_selected_row(&mut self) {
        let Some(text) = self.selected_value() else {
            return;
        };

        let message = match &mut self.clipboard {
            Some(clipboard) => match clipboard.set_text(text) {
                Ok(()) => "Copied selected value".to_string(),
                Err(error) => format!("Clipboard error: {error}"),
            },
            None => "Clipboard is unavailable".to_string(),
        };
        print!("{message}");
    }

    fn selected_value(&self) -> Option<String> {
        let Some(row) = self
            .table_state
            .selected()
            .and_then(|selected| self.items.get(selected))
        else {
            return None;
        };

        let Some(header) = self.selected_column_name() else {
            return None;
        };

        Some(
            row.get(&header)
                .map(Self::format_db_value)
                .unwrap_or_else(|| "null".to_string()),
        )
    }

    fn column_count(&self) -> usize {
        self.items
            .iter()
            .flat_map(|row| row.keys())
            .collect::<BTreeSet<_>>()
            .len()
            .max(1)
    }

    fn query_lines(&self) -> Vec<Line<'_>> {
        let subtitle_lines = if self.query_expanded {
            let mut query = vec![Line::from("Query:")];
            query.extend(
                self.query
                    .lines()
                    .map(|line| Line::from(line.to_string().yellow())),
            );
            query
        } else {
            vec![Line::from(vec![
                "Query [e to expand]: ".into(),
                self.query.replace(['\n', '\r'], " ").yellow(),
            ])]
        };

        subtitle_lines
    }

    fn expanded_value_line(&self) -> Option<Line<'static>> {
        self.selected_value()
            .filter(|_| self.value_expanded)
            .map(|value| Line::from(vec!["Value: ".into(), value.cyan()]))
    }

    fn header_height(&self, available_width: usize) -> u16 {
        let query_height = if self.query_expanded {
            self.query_lines()
                .iter()
                .map(|line| line.width().div_ceil(available_width).max(1))
                .sum::<usize>()
        } else {
            1
        };

        query_height.saturating_add(1).min(u16::MAX as usize) as u16
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let query_lines = self.query_lines();
        let value_line = self.expanded_value_line();

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

    fn items_to_rows_elements(&self) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = self
            .items
            .iter()
            .flat_map(|row| row.keys().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        if headers.is_empty() {
            return (
                vec!["result".to_string()],
                vec![vec!["No rows".to_string()]],
            );
        }

        let rows = self
            .items
            .iter()
            .map(|row| {
                headers
                    .iter()
                    .map(|header| {
                        row.get(header)
                            .map(Self::format_db_value)
                            .unwrap_or_else(|| "null".to_string())
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        (headers, rows)
    }

    fn render_table(&mut self, area: Rect, buf: &mut Buffer) {
        let (headers, row_values) = self.items_to_rows_elements();

        let visible_columns = ((area.width.saturating_add(COLUMN_SPACING))
            / (COLUMN_WIDTH + COLUMN_SPACING))
            .max(1) as usize;

        match self.selected_column {
            n if n < self.column_offset => self.column_offset = n,
            n if n >= self.column_offset + visible_columns => {
                self.column_offset = n.saturating_add(1).saturating_sub(visible_columns)
            }
            _ => {}
        }

        self.column_offset = self
            .column_offset
            .min(headers.len().saturating_sub(visible_columns));
        let visible_end = (self.column_offset + visible_columns).min(headers.len());
        let visible_headers = headers[self.column_offset..visible_end].to_vec();

        let header = Row::new(visible_headers.clone())
            .style(Style::new().bold())
            .bottom_margin(1);

        let rows = row_values.into_iter().map(|row| {
            Row::new(
                row.into_iter()
                    .skip(self.column_offset)
                    .take(visible_columns)
                    .collect::<Vec<_>>(),
            )
        });

        let widths = visible_headers
            .iter()
            .map(|_| Constraint::Length(COLUMN_WIDTH))
            .collect::<Vec<_>>();

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(COLUMN_SPACING)
            // .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Color::DarkGray)
            .cell_highlight_style(Style::new().reversed().yellow())
            .highlight_symbol("> ");

        self.table_state.select_column(Some(
            self.selected_column.saturating_sub(self.column_offset),
        ));
        StatefulWidget::render(table, area, buf, &mut self.table_state);
    }

    fn footer_commands(&self) -> Line<'static> {
        let commands: &[(&str, &str)] = match self.command {
            TableCommand::ShowTables => &[
                ("↑/↓ or j/k", "navigate"),
                ("Enter", "select"),
                ("y", "copy"),
                ("e", "query"),
                ("p", "edit"),
                ("q", "quit"),
            ],
            TableCommand::ShowValue => &[
                ("↑/↓/←/→ or j/k/h/l", "navigate"),
                ("Enter", "value"),
                ("y", "copy"),
                ("u", "update"),
                ("e", "query"),
                ("p", "edit"),
                ("q", "quit"),
            ],
        };

        Line::from(
            commands
                .iter()
                .flat_map(|&(key, action)| [key.yellow(), format!(" {action}  ").into()])
                .collect::<Vec<_>>(),
        )
        .dark_gray()
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let [_, duration_area, commands_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        Paragraph::new(Line::from(vec![
            "Duration: ".dark_gray(),
            format!("{:?}", self.duration).blue(),
        ]))
        .alignment(ratatui::layout::HorizontalAlignment::Left)
        .dark_gray()
        .render(duration_area, buf);

        Paragraph::new(self.footer_commands())
            .alignment(ratatui::layout::HorizontalAlignment::Center)
            .render(commands_area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let available_width = usize::from(area.width.max(1));

        let [header_area, table_area, footer_area] = Layout::vertical([
            Constraint::Length(self.header_height(available_width)),
            Constraint::Fill(1),
            Constraint::Length(FOOTER_HEIGHT),
        ])
        .areas(area);

        self.render_header(header_area, buf);
        self.render_table(table_area, buf);
        self.render_footer(footer_area, buf);
    }
}
