use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};

#[derive(Default)]
pub(crate) struct GotoPopup {
    visible: bool,
    input: String,
    error: Option<String>,
}

impl GotoPopup {
    pub(crate) fn open(&mut self) {
        self.visible = true;
        self.input.clear();
        self.error = None;
    }

    pub(crate) fn reset(&mut self) {
        self.close();
    }

    pub(crate) fn is_visible(&self) -> bool {
        self.visible
    }

    /// Handles modal input and returns a zero-based row index when submitted.
    pub(crate) fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        line_count: usize,
    ) -> Option<usize> {
        match key_event.code {
            KeyCode::Char(character) if character.is_ascii_digit() => {
                self.input.push(character);
                self.error = None;
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.error = None;
            }
            KeyCode::Enter => return self.submit(line_count),
            KeyCode::Esc | KeyCode::Char('q') => self.close(),
            _ => {}
        }

        None
    }

    fn submit(&mut self, line_count: usize) -> Option<usize> {
        let Ok(line_number) = self.input.parse::<usize>() else {
            self.error = Some("Enter a valid line number".to_string());
            return None;
        };

        if line_number == 0 || line_number > line_count {
            self.error = Some(format!("Line must be between 1 and {line_count}"));
            return None;
        }

        self.close();
        Some(line_number - 1)
    }

    fn close(&mut self) {
        self.visible = false;
        self.input.clear();
        self.error = None;
    }
}

impl Widget for &GotoPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        let popup_area = area.centered(Constraint::Length(40), Constraint::Length(4));
        Clear.render(popup_area, buf);

        let mut lines = vec![Line::from(format!("Line: {}▏", self.input))];
        if let Some(error) = &self.error {
            lines.push(Line::from(error.clone()));
        }

        Paragraph::new(lines)
            .block(Block::bordered().title(" Go to line "))
            .render(popup_area, buf);
    }
}
