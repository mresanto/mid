use std::collections::{BTreeSet, HashMap};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Row, StatefulWidget, Table, TableState},
};

use crate::core::databases::adapters::database_type::DbValue;

use super::format_db_value;

const COLUMN_WIDTH: u16 = 20;
const COLUMN_SPACING: u16 = 1;

pub(crate) struct ResultsTable<'a> {
    items: &'a [HashMap<String, DbValue>],
    selected_column: usize,
    column_offset: &'a mut usize,
}

impl<'a> ResultsTable<'a> {
    pub(crate) fn new(
        items: &'a [HashMap<String, DbValue>],
        selected_column: usize,
        column_offset: &'a mut usize,
    ) -> Self {
        Self {
            items,
            selected_column,
            column_offset,
        }
    }

    fn elements(&self) -> (Vec<String>, Vec<Vec<String>>) {
        let headers = self
            .items
            .iter()
            .flat_map(|row| row.keys().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        if headers.is_empty() {
            return (vec!["result".into()], vec![vec!["No rows".into()]]);
        }

        let rows = self
            .items
            .iter()
            .map(|row| {
                headers
                    .iter()
                    .map(|header| {
                        row.get(header)
                            .map_or_else(|| "null".into(), format_db_value)
                    })
                    .collect()
            })
            .collect();
        (headers, rows)
    }
}

impl StatefulWidget for ResultsTable<'_> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (headers, row_values) = self.elements();
        let row_number_width = self.items.len().max(1).to_string().len() as u16;
        let data_width = area.width.saturating_sub(row_number_width + COLUMN_SPACING);
        let visible_columns = ((data_width.saturating_add(COLUMN_SPACING))
            / (COLUMN_WIDTH + COLUMN_SPACING))
            .max(1) as usize;

        match self.selected_column {
            n if n < *self.column_offset => *self.column_offset = n,
            n if n >= *self.column_offset + visible_columns => {
                *self.column_offset = n.saturating_add(1).saturating_sub(visible_columns);
            }
            _ => {}
        }
        *self.column_offset =
            (*self.column_offset).min(headers.len().saturating_sub(visible_columns));

        let visible_end = (*self.column_offset + visible_columns).min(headers.len());
        let visible_headers = headers[*self.column_offset..visible_end].to_vec();
        let mut table_headers = Vec::with_capacity(visible_headers.len() + 1);
        table_headers.push("#".to_string());
        table_headers.extend(visible_headers.clone());
        let header = Row::new(table_headers)
            .style(Style::new().bold())
            .bottom_margin(1);

        let has_items = !self.items.is_empty();
        let rows = row_values.into_iter().enumerate().map(|(index, row)| {
            let mut cells = Vec::with_capacity(visible_columns + 1);
            cells.push(if has_items {
                (index + 1).to_string()
            } else {
                String::new()
            });
            cells.extend(
                row.into_iter()
                    .skip(*self.column_offset)
                    .take(visible_columns),
            );
            Row::new(cells)
        });

        let mut widths = Vec::with_capacity(visible_headers.len() + 1);
        widths.push(Constraint::Length(row_number_width));
        widths.extend(
            visible_headers
                .iter()
                .map(|_| Constraint::Length(COLUMN_WIDTH)),
        );

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(COLUMN_SPACING)
            .column_highlight_style(Color::DarkGray)
            .cell_highlight_style(Style::new().reversed().yellow())
            .highlight_symbol("> ");

        state.select_column(Some(
            self.selected_column
                .saturating_sub(*self.column_offset)
                .saturating_add(1),
        ));
        StatefulWidget::render(table, area, buf, state);
    }
}
