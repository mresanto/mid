use std::{
    collections::{BTreeSet, HashMap},
    io,
    time::{Duration, Instant},
};

use arboard::Clipboard;
use crossterm::{
    event::{
        self, Event, KeyEvent, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    terminal::supports_keyboard_enhancement,
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{StatefulWidget, TableState, Widget},
};

use super::query_components::{
    FilterPopup, Footer, GotoPopup, ResultsTable, ResultsTableData, format_db_value,
};

use crate::core::{
    config::manage,
    databases::adapters::database_type::{DatabaseHandler, DbValue},
    globals,
    query::{TableCommand, TableEvent, formats::app::keybinds_events::KeybindEvents},
};

struct KeyboardEnhancementGuard {
    enabled: bool,
}

impl KeyboardEnhancementGuard {
    fn enable() -> io::Result<Self> {
        let enabled = matches!(supports_keyboard_enhancement(), Ok(true));

        if enabled {
            crossterm::execute!(
                io::stdout(),
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                )
            )?;
        }

        Ok(Self { enabled })
    }
}

impl Drop for KeyboardEnhancementGuard {
    fn drop(&mut self) {
        if self.enabled {
            let _ = crossterm::execute!(io::stdout(), PopKeyboardEnhancementFlags);
        }
    }
}

#[derive(Default)]
pub struct QueryScreen {
    items: Vec<HashMap<String, DbValue>>,
    visible_indices: Vec<usize>,
    table_data: ResultsTableData,
    command: TableCommand,
    query: String,
    exit: bool,
    table_state: TableState,
    column_offset: usize,
    selected_column: usize,
    clipboard: Option<Clipboard>,
    event: Option<TableEvent>,
    duration: Duration,
    goto_popup: GotoPopup,
    filter_popup: FilterPopup,
    copied_cell: Option<(usize, usize, Instant)>,
    select_mode: bool,
    select_values: Vec<(usize, usize)>,
}

impl QueryScreen {
    pub fn new(items: Vec<HashMap<String, DbValue>>, command: TableCommand, query: String) -> Self {
        let mut table_state = TableState::default();
        if !items.is_empty() {
            table_state.select_first();
        }
        let table_data = ResultsTableData::new(&items);
        let visible_indices = (0..items.len()).collect();

        Self {
            items,
            visible_indices,
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
        self.visible_indices = (0..items.len()).collect();
        self.table_data = ResultsTableData::new_filtered(&items, &self.visible_indices);
        self.items = items;
        self.query = query;
        self.duration = duration;
        self.exit = false;
        self.event = None;
        self.goto_popup.reset();
        self.filter_popup.reset();
        self.copied_cell = None;
        self.select_mode = false;
        self.select_values.clear();
        self.column_offset = 0;
        self.selected_column = 0;
        self.table_state = TableState::default();
        if !self.items.is_empty() {
            self.table_state.select_first();
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<Option<TableEvent>> {
        let _keyboard_enhancement = KeyboardEnhancementGuard::enable()?;

        while !self.exit {
            self.expire_copied_highlight();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(self.event.take())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Some((_, _, deadline)) = self.copied_cell {
            let timeout = deadline.saturating_duration_since(Instant::now());
            if !event::poll(timeout)? {
                return Ok(());
            }
        }

        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press
                    || (key_event.kind == KeyEventKind::Repeat
                        && !key_event
                            .modifiers
                            .intersects(KeyModifiers::SHIFT | KeyModifiers::CONTROL)) =>
            {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.goto_popup.is_visible() {
            if let Some(index) = self
                .goto_popup
                .handle_key_event(key_event, self.visible_indices.len())
            {
                self.table_state.select(Some(index));
            }
            return;
        }

        if self.filter_popup.is_visible() {
            if let Some(filter) = self.filter_popup.handle_key_event(key_event) {
                self.apply_filter(&filter);
            }
            return;
        }

        if self.select_mode {
            if let Some(event) = KeybindEvents::parse_to_event(&key_event, &self.command) {
                match event {
                    KeybindEvents::NextRow => self.select_next_row(),
                    KeybindEvents::PreviousRow => self.select_previous_row(),
                    KeybindEvents::NextColumn => self.select_next_column(),
                    KeybindEvents::PreviousColumn => self.select_previous_column(),
                    KeybindEvents::FirstRow => self.select_first_row(),
                    KeybindEvents::LastRow => self.select_last_row(),
                    KeybindEvents::Quit => self.exit(),
                    KeybindEvents::OpenValue => self.select_value(),
                    KeybindEvents::SelectMode => self.select_mode(),
                    KeybindEvents::OpenValueInSelectMode => self.open_value_in_select_mode(),
                    _ => {}
                }
            }
            return;
        }

        if let Some(event) = KeybindEvents::parse_to_event(&key_event, &self.command) {
            match event {
                KeybindEvents::NextRow => self.select_next_row(),
                KeybindEvents::PreviousRow => self.select_previous_row(),
                KeybindEvents::NextColumn => self.select_next_column(),
                KeybindEvents::PreviousColumn => self.select_previous_column(),
                KeybindEvents::FirstRow => self.select_first_row(),
                KeybindEvents::LastRow => self.select_last_row(),
                KeybindEvents::YankSelection => self.yank_selected_row(),
                KeybindEvents::UpdateSelection => self.update_selected_value(),
                KeybindEvents::EditQuery => self.edit_query(),
                KeybindEvents::TableSearch => self.select_table(),
                KeybindEvents::GoToRow => self.goto_index(),
                KeybindEvents::Filter => self.filter_popup.open(),
                KeybindEvents::OpenValue => self.open_selected_row(),
                KeybindEvents::Quit => self.exit(),
                KeybindEvents::SelectMode => self.select_mode(),
                _ => {}
            }
        }
    }

    fn open_value_in_select_mode(&mut self) {
        let Some(text) = self.selected_values_text() else {
            return;
        };
        self.event = Some(TableEvent::OpenSelectedRow(text));
        self.exit();
    }

    fn select_mode(&mut self) {
        self.select_mode = !self.select_mode;

        if !self.select_mode {
            self.select_values.clear();
        }
    }

    fn select_value(&mut self) {
        if self.select_mode {
            if let Some(index) = self.selected_item_index() {
                let selected_value = (index, self.selected_column);
                if let Some(position) = self
                    .select_values
                    .iter()
                    .position(|value| *value == selected_value)
                {
                    self.select_values.remove(position);
                } else {
                    self.select_values.push(selected_value);
                }
            }
        }
    }

    fn goto_index(&mut self) {
        self.goto_popup.open();
    }

    fn apply_filter(&mut self, filter: &str) {
        let Some(column) = self.selected_column_name() else {
            return;
        };
        let filter = filter.to_lowercase();
        self.visible_indices = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, row)| {
                let value = row
                    .get(&column)
                    .map_or_else(|| "null".to_string(), format_db_value);
                (filter.is_empty() || value.to_lowercase().contains(filter.as_str()))
                    .then_some(index)
            })
            .collect();
        self.table_data = ResultsTableData::new_filtered(&self.items, &self.visible_indices);
        self.table_state = TableState::default();
        if !self.visible_indices.is_empty() {
            self.table_state.select_first();
        }
        //self.value_expanded = false;
        self.copied_cell = None;
    }

    fn selected_item_index(&self) -> Option<usize> {
        self.table_state
            .selected()
            .and_then(|selected| self.visible_indices.get(selected).copied())
    }

    fn edit_query(&mut self) {
        self.event = Some(TableEvent::EditQuery(self.query.clone()));
        self.exit();
    }

    fn update_selected_value(&mut self) {
        let Some(selected_row) = self
            .selected_item_index()
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

        let file_path = globals::get_global_config_file_path();
        let config = manage::read_config(file_path).unwrap();
        let database = config.get_database_type().unwrap();
        let update_query = database.update(&table, &id_column, id, &column, value);
        self.event = Some(TableEvent::UpdateValue(format!(
            "-- Save and close to apply this update.\n\
            -- Delete all file to cancel.\n\n\
            {}",
            update_query
        )));
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

    fn select_table(&mut self) {
        let Some(table_name) = self
            .selected_item_index()
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

    fn select_next_row(&mut self) {
        if self.visible_indices.is_empty() {
            return;
        }

        let current_row = self.table_state.selected().unwrap_or(0);
        let last_row = self.visible_indices.len().saturating_sub(1);
        self.table_state
            .select(Some(current_row.saturating_add(1).min(last_row)));
    }

    fn select_previous_row(&mut self) {
        if self.visible_indices.is_empty() {
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

    fn open_selected_row(&mut self) {
        let Some(text) = self.selected_value() else {
            return;
        };
        self.event = Some(TableEvent::OpenSelectedRow(text));
        self.exit();
    }

    fn select_first_row(&mut self) {
        if !self.visible_indices.is_empty() {
            self.table_state.select_first();
        }
    }

    fn select_last_row(&mut self) {
        if !self.visible_indices.is_empty() {
            self.table_state.select_last();
        }
    }

    fn yank_selected_row(&mut self) {
        let Some(text) = self.selected_value() else {
            return;
        };

        let copied = match &mut self.clipboard {
            Some(clipboard) => match clipboard.set_text(text) {
                Ok(()) => true,
                Err(_) => false,
            },
            None => false,
        };

        if copied {
            if let Some(row) = self.table_state.selected() {
                self.copied_cell = Some((
                    row,
                    self.selected_column,
                    Instant::now() + Duration::from_millis(500),
                ));
            }
        }
    }

    fn expire_copied_highlight(&mut self) {
        if self
            .copied_cell
            .is_some_and(|(_, _, deadline)| Instant::now() >= deadline)
        {
            self.copied_cell = None;
        }
    }

    fn copied_cell(&self) -> Option<(usize, usize)> {
        self.copied_cell.map(|(row, column, _)| (row, column))
    }

    fn selected_value(&self) -> Option<String> {
        let Some(row) = self
            .selected_item_index()
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

    fn selected_values_text(&self) -> Option<String> {
        if self.select_values.is_empty() {
            return None;
        }

        let headers = self
            .items
            .iter()
            .flat_map(|row| row.keys().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let mut selected_values = self.select_values.clone();
        selected_values.sort_unstable();

        let mut rows = Vec::new();
        let mut current_row = None;
        let mut row_values = Vec::new();

        for (row_index, column_index) in selected_values {
            if current_row.is_some_and(|current| current != row_index) {
                rows.push(row_values.join(" | "));
                row_values.clear();
            }
            current_row = Some(row_index);

            let value = self
                .items
                .get(row_index)
                .and_then(|row| headers.get(column_index).and_then(|header| row.get(header)))
                .map(format_db_value)
                .unwrap_or_else(|| "null".to_string());
            row_values.push(value);
        }

        if !row_values.is_empty() {
            rows.push(row_values.join(" | "));
        }

        Some(rows.join("\n"))
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
        let copied_cell = self.copied_cell();

        let [table_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(Footer::HEIGHT)]).areas(area);

        StatefulWidget::render(
            ResultsTable::new(
                &self.table_data,
                self.selected_column,
                &mut self.column_offset,
                copied_cell,
                &self.select_values,
            ),
            table_area,
            buf,
            &mut self.table_state,
        );
        Footer::new(
            &self.command,
            self.duration,
            &self.items,
            self.table_data.item_count(),
            self.select_mode,
            self.select_values.len(),
        )
        .render(footer_area, buf);

        (&self.goto_popup).render(area, buf);
        (&self.filter_popup).render(area, buf);
    }
}
