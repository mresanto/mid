use std::collections::{BTreeSet, HashMap};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Row, Table, TableState};

use crate::core::databases::application::query::DbValue;

pub fn render_outout_as_table(items: Vec<HashMap<String, DbValue>>) -> Result<()> {
    color_eyre::install()?;

    let mut table_state = TableState::default();
    table_state.select_first();
    table_state.select_first_column();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &mut table_state, &items))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => table_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => table_state.select_previous(),
                    KeyCode::Char('l') | KeyCode::Right => table_state.select_next_column(),
                    KeyCode::Char('h') | KeyCode::Left => table_state.select_previous_column(),
                    KeyCode::Char('g') => table_state.select_first(),
                    KeyCode::Char('G') => table_state.select_last(),
                    _ => {}
                }
            }
        }
    })
}

/// Render the UI with a table.
fn render(frame: &mut Frame, table_state: &mut TableState, items: &Vec<HashMap<String, DbValue>>) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("Table Widget").bold(),
        Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_table(frame, main, table_state, items);
}

/// Render a table with some rows and columns.
pub fn render_table(
    frame: &mut Frame,
    area: Rect,
    table_state: &mut TableState,
    items: &Vec<HashMap<String, DbValue>>,
) {
    let (headers, row_values) = format_to_table_elements(items);

    let header = Row::new(headers.clone())
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows: Vec<Row> = row_values.into_iter().map(Row::new).collect();
    let widths: Vec<Constraint> = headers.iter().map(|_| Constraint::Fill(1)).collect();

    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("🍴 ");

    frame.render_stateful_widget(table, area, table_state);
}

fn format_to_table_elements(items: &[HashMap<String, DbValue>]) -> (Vec<String>, Vec<Vec<String>>) {
    let headers: Vec<String> = items
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

    let rows = items
        .iter()
        .map(|row| {
            headers
                .iter()
                .map(|header| {
                    row.get(header)
                        .map(format_db_value)
                        .unwrap_or_else(|| "null".to_string())
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    (headers, rows)
}

fn format_db_value(value: &DbValue) -> String {
    match value {
        DbValue::Null => "null".to_string(),
        DbValue::Text(value) => value.clone(),
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
