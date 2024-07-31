use std::{cmp::Ordering, io::stdout};

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

use crate::template::Template;

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

struct State {
    matching_templates: Vec<(Template, Vec<usize>)>,
    search_term: String,
    list_state: ListState,
}

pub fn pick_template(templates: &[Template]) -> anyhow::Result<Option<Template>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let state = Mutex::new(State {
        matching_templates: templates.iter().map(|t| (t.clone(), vec![])).collect(),
        search_term: String::new(),
        list_state: {
            let mut state = ListState::default();
            state.select(Some(0));
            state
        },
    });

    let ui = |frame: &mut Frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(frame.size());

        let text_input = Paragraph::new(state.lock().search_term.clone().cyan())
            .block(Block::bordered().title("Fuzzy Search"))
            .style(Style::default().fg(Color::White));

        frame.render_widget(text_input, chunks[0]);

        let list = List::new(state.lock().matching_templates.iter().map(|(t, indices)| {
            Line::from(indices_template(t, indices)).add_modifier(Modifier::DIM)
        }))
        .block(
            Block::bordered()
                .title("Templates")
                .title_bottom("<Ctrl+C> to quit | <Up/Down> to navigate | <Enter> to select"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().remove_modifier(Modifier::DIM))
        .highlight_symbol("> ");

        frame.render_stateful_widget(list, chunks[1], &mut state.lock().list_state);
    };

    let selected = loop {
        terminal.draw(ui)?;
        let (should_quit, selected) = handle_events(templates, &state)?;

        if should_quit {
            break selected;
        }
    };

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(selected)
}

fn handle_events(
    templates: &[Template],
    state: &Mutex<State>,
) -> anyhow::Result<(bool, Option<Template>)> {
    let mut state = state.lock();

    let mut should_quit = false;

    let current_index = state.list_state.selected().unwrap_or(0);

    state.matching_templates = templates
        .iter()
        .filter_map(|t| {
            SkimMatcherV2::default()
                .fuzzy_indices(t.name(), state.search_term.as_str())
                .map(|(score, indices)| (t, score, indices))
        })
        .sorted_by(
            |(a, score_a, _), (b, score_b, _)| match score_b.cmp(score_a) {
                Ordering::Equal => a.cmp(b),
                ordering => ordering,
            },
        )
        .map(|(t, _, indices)| (t.clone(), indices))
        .collect();

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Adjustment {
        Up,
        Down,
    }

    let templates_len = {
        let matching_templates = &state.matching_templates;
        matching_templates.len()
    };

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
                KeyCode::Enter => {
                    let selected = state.list_state.selected();

                    if let Some(selected) = selected {
                        return Ok((true, Some(templates[selected].clone())));
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
