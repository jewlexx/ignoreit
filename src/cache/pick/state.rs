use std::{
    cmp::Ordering,
    hash::{self, Hash, Hasher},
};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use parking_lot::{Mutex, MutexGuard};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    widgets::ListState,
};

use crate::{
    cache::{Folder, Item},
    template::Template,
};

pub type History = Vec<HistoryEntry>;

static STATE_HASH: Mutex<u64> = Mutex::new(0);
static MATCHING_TEMPLATES: Mutex<Vec<(Item, Vec<usize>)>> = Mutex::new(Vec::new());

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryEntry {
    pub folder: Folder,
    pub selection: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
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
    pub fn state_changed(&self) -> bool {
        self.get_hash() != *STATE_HASH.lock()
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = hash::DefaultHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }

    pub fn update_hash(&self) {
        *STATE_HASH.lock() = self.get_hash();
    }

    pub fn list_matching_templates<'a>() -> MutexGuard<'a, Vec<(Item, Vec<usize>)>> {
        MATCHING_TEMPLATES.lock()
    }

    pub fn update_matching_templates(&self) {
        if !self.state_changed() {
            return;
        }

        *MATCHING_TEMPLATES.lock() = self
            .current_folder
            .list_items()
            .iter()
            .filter_map(|t| {
                SkimMatcherV2::default()
                    .fuzzy_indices(t.name(), self.search_term.as_str())
                    .map(|(score, indices)| (t, score, indices))
            })
            .sorted_by(
                |(a, score_a, _), (b, score_b, _)| match score_b.cmp(score_a) {
                    Ordering::Equal => a.name().to_lowercase().cmp(&b.name().to_lowercase()),
                    ordering => ordering,
                },
            )
            .map(|(t, _, indices)| (t.clone(), indices))
            .collect();
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
