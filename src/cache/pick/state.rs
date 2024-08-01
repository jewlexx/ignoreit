use ratatui::widgets::ListState;

use crate::cache::Folder;

pub type History = Vec<HistoryEntry>;

pub struct HistoryEntry {
    pub folder: Folder,
    pub selection: Option<usize>,
}

pub struct State {
    // matching_templates: Vec<(Template, Vec<usize>)>,
    pub search_term: String,
    pub list_state: ListState,
    pub current_folder: Folder,
    pub history: History,
}
