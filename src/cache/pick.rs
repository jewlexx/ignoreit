mod state;
mod ui;

use std::{io::stdout, rc::Rc};

use parking_lot::Mutex;
use ratatui::{
    crossterm::{
        event::{self, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use state::State;

use crate::{template::Template, CACHE};

pub fn pick_template() -> anyhow::Result<Option<Template>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let state = Rc::new(Mutex::new(State {
        // matching_templates: templates.iter().map(|t| (t.clone(), vec![])).collect(),
        search_term: String::new(),
        list_state: {
            let mut state = ListState::default();
            state.select(Some(0));
            state
        },
        current_folder: CACHE.root.clone(),
        history: Vec::new(),
    }));

    state.lock().update_hash();

    let selected = loop {
        terminal.draw(ui::ui(state.clone()))?;
        let (should_quit, selected) = handle_events(&state)?;

        if should_quit {
            break selected;
        }
    };

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(selected)
}

fn handle_events(state: &Mutex<State>) -> anyhow::Result<(bool, Option<Template>)> {
    let mut state = state.lock();

    if let Event::Key(key) = event::read()? {
        Ok(state.handle_key_event(key))
    } else {
        Ok((false, None))
    }
}
