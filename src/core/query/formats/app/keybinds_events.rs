use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::core::query::TableCommand;

#[derive(Clone, Copy)]
pub enum KeybindEvents {
    NextRow,
    PreviousRow,
    NextColumn,
    PreviousColumn,
    FirstRow,
    LastRow,
    YankSelection,
    UpdateSelection,
    ToggleQuery,
    EditQuery,
    TableSearch,
    GoToRow,
    Filter,
    OpenValue,
    Quit,
}

impl KeybindEvents {
    pub fn parse_to_event(input: &KeyEvent, command: &TableCommand) -> Option<Self> {
        match input.code {
            KeyCode::Up | KeyCode::Char('j') => Some(Self::PreviousRow),
            KeyCode::Down | KeyCode::Char('k') => Some(Self::NextRow),
            KeyCode::Left | KeyCode::Char('h') => Some(Self::PreviousColumn),
            KeyCode::Right | KeyCode::Char('l') => Some(Self::NextColumn),
            KeyCode::Char('g') if input.modifiers.contains(KeyModifiers::SHIFT) => {
                Some(Self::GoToRow)
            }
            KeyCode::Home | KeyCode::Char('g') => Some(Self::FirstRow),
            KeyCode::End | KeyCode::Char('G') => Some(Self::LastRow),
            KeyCode::Char('y') => Some(Self::YankSelection),
            KeyCode::Char('q') => Some(Self::Quit),
            KeyCode::Char('u') => Some(Self::UpdateSelection),
            KeyCode::Char('t') => Some(Self::ToggleQuery),
            KeyCode::Char('e') => Some(Self::EditQuery),
            KeyCode::Enter => match command {
                TableCommand::ShowTables => Some(Self::TableSearch),
                TableCommand::ShowValue => Some(Self::OpenValue),
            },
            KeyCode::Char('f') => Some(Self::Filter),
            KeyCode::Char('o') => Some(Self::OpenValue),
            _ => None,
        }
    }

    pub fn parse_to_command(&self) -> &'static str {
        match self {
            Self::NextRow => "j",
            Self::PreviousRow => "k",
            Self::NextColumn => "l",
            Self::PreviousColumn => "h",
            Self::FirstRow => "g",
            Self::LastRow => "G",
            Self::YankSelection => "y",
            Self::UpdateSelection => "u",
            Self::ToggleQuery => "t",
            Self::EditQuery => "e",
            Self::TableSearch => "Enter",
            Self::Filter => "f",
            Self::OpenValue => "Enter",
            Self::GoToRow => "Shift+g",
            Self::Quit => "q",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::NextRow => "next row",
            Self::PreviousRow => "previous row",
            Self::NextColumn => "next column",
            Self::PreviousColumn => "previous column",
            Self::FirstRow => "first row",
            Self::LastRow => "last row",
            Self::YankSelection => "yank",
            Self::UpdateSelection => "update",
            Self::ToggleQuery => "query",
            Self::EditQuery => "edit",
            Self::TableSearch => "select",
            Self::GoToRow => "go to",
            Self::Filter => "filter",
            Self::OpenValue => "value",
            Self::Quit => "quit",
        }
    }

    pub fn footer_events(command: &TableCommand) -> &'static [Self] {
        const TABLE_EVENTS: &[KeybindEvents] = &[
            KeybindEvents::NextRow,
            KeybindEvents::PreviousRow,
            KeybindEvents::GoToRow,
            KeybindEvents::Filter,
            KeybindEvents::TableSearch,
            KeybindEvents::YankSelection,
            KeybindEvents::ToggleQuery,
            KeybindEvents::EditQuery,
            KeybindEvents::Quit,
        ];
        const VALUE_EVENTS: &[KeybindEvents] = &[
            KeybindEvents::NextRow,
            KeybindEvents::PreviousRow,
            KeybindEvents::NextColumn,
            KeybindEvents::PreviousColumn,
            KeybindEvents::GoToRow,
            KeybindEvents::Filter,
            KeybindEvents::OpenValue,
            KeybindEvents::YankSelection,
            KeybindEvents::UpdateSelection,
            KeybindEvents::ToggleQuery,
            KeybindEvents::EditQuery,
            KeybindEvents::Quit,
        ];

        match command {
            TableCommand::ShowTables => TABLE_EVENTS,
            TableCommand::ShowValue => VALUE_EVENTS,
        }
    }
}
