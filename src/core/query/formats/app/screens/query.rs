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
    widgets::{StatefulWidget, TableState, Widget},
};

use crate::core::{
    databases::{
        adapters::database_type::DbValue, application::tables::update::update_database_table,
    },
    query::{TableCommand, TableEvent},
};

use super::super::components::{Footer, Header, ResultsTable, ResultsTableData, format_db_value};

#[derive(Default)]
pub struct QueryScreen {
    items: Vec<HashMap<String, DbValue>>,
    table_data: ResultsTableData,
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

impl QueryScreen {
    pub fn new(items: Vec<HashMap<String, DbValue>>, command: TableCommand, query: String) -> Self {
        let mut table_state = TableState::default();
        if !items.is_empty() {
            table_state.select_first();
        }
        let table_data = ResultsTableData::new(&items);

        Self {
            items,
            table_data,
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
        self.table_data = ResultsTableData::new(&items);
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
            .map(format_db_value)
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
        let has_selection = self.table_state.selected().is_some_and(|selected| {
            self.items.get(selected).is_some() && self.selected_column_name().is_some()
        });
        if has_selection {
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
                .map(format_db_value)
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
}

impl Widget for &mut QueryScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let available_width = usize::from(area.width.max(1));
        let selected_value = self.value_expanded.then(|| self.selected_value()).flatten();
        let header = Header::new(
            &self.query,
            self.query_expanded,
            selected_value,
            self.value_expanded,
        );

        let [header_area, table_area, footer_area] = Layout::vertical([
            Constraint::Length(header.height(available_width)),
            Constraint::Fill(1),
            Constraint::Length(Footer::HEIGHT),
        ])
        .areas(area);

        header.render(header_area, buf);
        StatefulWidget::render(
            ResultsTable::new(
                &self.table_data,
                self.selected_column,
                &mut self.column_offset,
            ),
            table_area,
            buf,
            &mut self.table_state,
        );
        Footer::new(&self.command, self.duration, &self.items).render(footer_area, buf);
    }
}
