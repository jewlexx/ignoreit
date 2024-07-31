use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    hint::unreachable_unchecked,
    io::stdout,
    rc::Rc,
};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use parking_lot::Mutex;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};

use crate::{
    cache::Cache,
    template::{Category, Template},
    CACHE,
};

use super::{Folder, Item};

fn indices_template<'a>(template: &Template, indices: &[usize]) -> Vec<Span<'a>> {
    let template_name = template.to_string();
    let chars = template_name.chars().collect::<Vec<_>>();

    let mut spans = Vec::new();

    for (i, c) in chars.iter().enumerate() {
        if indices.contains(&i) {
            spans.push(Span::styled(
                c.to_string(),
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::raw(c.to_string()));
        }
    }

    spans
}

struct PastFolder {
    folder: Folder,
    selection: Option<usize>,
}

struct State {
    // matching_templates: Vec<(Template, Vec<usize>)>,
    search_term: String,
    list_state: ListState,
    current_folder: Folder,
    past_folders: Vec<PastFolder>,
}

pub fn pick_template() -> anyhow::Result<Option<Template>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let state = Mutex::new(State {
        // matching_templates: templates.iter().map(|t| (t.clone(), vec![])).collect(),
        search_term: String::new(),
        list_state: {
            let mut state = ListState::default();
            state.select(Some(0));
            state
        },
        current_folder: CACHE.root.clone(),
        past_folders: Vec::new(),
    });

    let ui = |frame: &mut Frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(2),
                    Constraint::Percentage(10),
                    Constraint::Percentage(88),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let folder_title = {
            let state = &state.lock();
            let current_folder = &state.current_folder;
            let previous_folders = &state.past_folders;

            let mut breadcrumbs = String::new();

            for folder in previous_folders {
                breadcrumbs.push_str(&format!("{}/", folder.folder.name))
            }

            Paragraph::new(Line::from(vec![
                crate::icons::FOLDER_OPEN.to_string().into(),
                " ".into(),
                breadcrumbs.into(),
                current_folder.name.clone().into(),
            ]))
        };

        let text_input = Paragraph::new(state.lock().search_term.clone().cyan())
            .block(Block::bordered().title("Fuzzy Search"))
            .style(Style::default().fg(Color::White));

        let items = state.lock().current_folder.list_items();

        let list = List::new(items.iter().map(|t| {
            let spans = vec![
                Span::raw(t.get_icon().to_string()),
                Span::raw(" "),
                Span::raw(t.name()),
            ];
            // spans.extend(indices_template(t, indices));

            Line::from(spans).add_modifier(Modifier::DIM)
        }))
        .block(
            Block::bordered()
                .title("Templates")
                .title_bottom("<Ctrl+C> to quit | <Up/Down> to navigate | <Enter> to select | <Left | Right Arrows> to navigate folders"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().remove_modifier(Modifier::DIM))
        .highlight_symbol("> ");

        frame.render_widget(folder_title, chunks[0]);
        frame.render_widget(text_input, chunks[1]);
        frame.render_stateful_widget(list, chunks[2], &mut state.lock().list_state);
    };

    let selected = loop {
        terminal.draw(ui)?;
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

    let mut should_quit = false;

    let current_index = state.list_state.selected().unwrap_or(0);

    // state.matching_templates = templates
    //     .iter()
    //     .filter_map(|t| {
    //         SkimMatcherV2::default()
    //             .fuzzy_indices(t.name(), state.search_term.as_str())
    //             .map(|(score, indices)| (t, score, indices))
    //     })
    //     .sorted_by(
    //         |(a, score_a, _), (b, score_b, _)| match score_b.cmp(score_a) {
    //             Ordering::Equal => a.cmp(b),
    //             ordering => ordering,
    //         },
    //     )
    //     .map(|(t, _, indices)| (t.clone(), indices))
    //     .collect();

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Adjustment {
        Up,
        Down,
    }

    let templates_len = state.current_folder.list_items().len();

    let adjust_index = |index: usize, adjustment: Adjustment| {
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
    };

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    should_quit = true
                }
                KeyCode::Up => state
                    .list_state
                    .select(Some(adjust_index(current_index, Adjustment::Down))),
                KeyCode::Down => state
                    .list_state
                    .select(Some(adjust_index(current_index, Adjustment::Up))),
                KeyCode::Enter | KeyCode::Right => {
                    let selected = state.list_state.selected();

                    if let Some(selected) = selected {
                        let item = state.current_folder.list_items()[selected].clone();

                        match item {
                            Item::Folder(mut folder) => {
                                let old_folder = {
                                    std::mem::swap(&mut state.current_folder, &mut folder);
                                    folder
                                };

                                let selection = state.list_state.selected();

                                state.past_folders.push(PastFolder {
                                    folder: old_folder,
                                    selection,
                                });
                            }
                            Item::Template(template) => {
                                if key.code != KeyCode::Right {
                                    return Ok((true, Some(template)));
                                }
                            }
                        }
                    }
                }
                KeyCode::Left => {
                    if let Some(PastFolder { folder, selection }) = state.past_folders.pop() {
                        state.current_folder = folder;
                        state.list_state.select(selection);
                    }
                }
                KeyCode::Backspace => {
                    let search_term = &mut state.search_term;

                    if !search_term.is_empty() {
                        search_term.pop();
                    }
                }
                KeyCode::Char(c) => {
                    let search_term = &mut state.search_term;

                    search_term.push(c);
                }
                _ => {}
            }
        }
    }

    Ok((should_quit, None))
}
