use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
};

use crate::core::query::formats::app::keybinds_events::KeybindEvents;

#[derive(Default)]
pub(crate) struct QueryPopup {
    visible: bool,
    copied: bool,
}

impl QueryPopup {
    pub(crate) fn open(&mut self) {
        self.visible = true;
        self.copied = false;
    }

    pub(crate) fn reset(&mut self) {
        self.visible = false;
        self.copied = false;
    }

    pub(crate) fn is_visible(&self) -> bool {
        self.visible
    }

    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('y') => return true,
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('t') => {
                self.visible = false;
            }
            _ => {}
        }

        false
    }

    pub(crate) fn set_copied(&mut self, copied: bool) {
        self.copied = copied;
    }
}

pub(crate) struct QueryPopupView<'a> {
    popup: &'a QueryPopup,
    query: &'a str,
}

impl<'a> QueryPopupView<'a> {
    pub(crate) fn new(popup: &'a QueryPopup, query: &'a str) -> Self {
        Self { popup, query }
    }
}

impl Widget for QueryPopupView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.popup.visible {
            return;
        }

        let popup_area = area.centered(Constraint::Percentage(80), Constraint::Percentage(80));
        Clear.render(popup_area, buf);
        let yank = KeybindEvents::YankSelection;
        let quit = KeybindEvents::Quit;
        let status = if self.popup.copied { "copied — " } else { "" };
        let title = format!(
            " Query — {status}{} {} — {} {} ",
            yank.parse_to_command(),
            yank.label(),
            quit.parse_to_command(),
            quit.label(),
        );
        Paragraph::new(self.query)
            .wrap(Wrap { trim: false })
            .block(Block::bordered().title(title))
            .render(popup_area, buf);
    }
}
