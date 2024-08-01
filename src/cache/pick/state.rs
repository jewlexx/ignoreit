use std::hash::{self, Hash, Hasher};

use parking_lot::Mutex;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    widgets::ListState,
};

use crate::{
    cache::{Folder, Item},
    template::Template,
};

pub type History = Vec<HistoryEntry>;

pub static STATE_HASH: Mutex<u64> = Mutex::new(0);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryEntry {
    pub folder: Folder,
    pub selection: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    // matching_templates: Vec<(Template, Vec<usize>)>,
    pub search_term: String,
    pub list_state: ListState,
    pub current_folder: Folder,
    pub history: History,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Adjustment {
    Up,
    Down,
}

impl State {
    pub fn update_hash(&self) {
        let mut hasher = hash::DefaultHasher::new();
        self.hash(&mut hasher);

        *STATE_HASH.lock() = hasher.finish();
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> (bool, Option<Template>) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return (true, None);
                }
                KeyCode::Up => self
                    .list_state
                    .select(Some(self.adjust_index(Adjustment::Down))),
                KeyCode::Down => self
                    .list_state
                    .select(Some(self.adjust_index(Adjustment::Up))),
                KeyCode::Enter | KeyCode::Right => {
                    let selected = self.list_state.selected();

                    if let Some(selected) = selected {
                        let item = self.current_folder.list_items()[selected].clone();

                        match item {
                            Item::Folder(mut folder) => {
                                let old_folder = {
                                    std::mem::swap(&mut self.current_folder, &mut folder);
                                    folder
                                };

                                let selection = self.list_state.selected();

                                self.history.push(HistoryEntry {
                                    folder: old_folder,
                                    selection,
                                });
                            }
                            Item::Template(template) => {
                                if key.code != KeyCode::Right {
                                    return (true, Some(template));
                                }
                            }
                        }
                    }
                }
                KeyCode::Left => {
                    if let Some(HistoryEntry { folder, selection }) = self.history.pop() {
                        self.current_folder = folder;
                        self.list_state.select(selection);
                    }
                }
                KeyCode::Backspace => {
                    let search_term = &mut self.search_term;

                    if !search_term.is_empty() {
                        search_term.pop();
                    }
                }
                KeyCode::Char(c) => {
                    let search_term = &mut self.search_term;

                    search_term.push(c);
                }
                _ => {}
            }
        }

        (false, None)
    }

    fn adjust_index(&self, adjustment: Adjustment) -> usize {
        let index = self.list_state.selected().unwrap_or_default();
        let templates_len = self.current_folder.list_items().len();

        if index == 0 && adjustment == Adjustment::Down {
            templates_len - 1
        } else if index == templates_len - 1 && adjustment == Adjustment::Up {
            0
        } else {
            match adjustment {
                Adjustment::Up => index + 1,
                Adjustment::Down => index - 1,
            }
        }
    }
}
