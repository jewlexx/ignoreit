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

use crate::{cache::Item, template::Template};

pub fn ui(state: Rc<Mutex<super::State>>) -> impl Fn(&mut ratatui::Frame<'_>) {
    let state = state.clone();
    move |frame: &mut Frame| {
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
            fn folder_name(name: &str) -> &str {
                const REAL_ROOT_NAME: &str = "templates";
                const NICE_ROOT_NAME: &str = "Gitignore Templates";

                if name == REAL_ROOT_NAME {
                    NICE_ROOT_NAME
                } else {
                    name
                }
            }

            let state = &state.lock();
            let breadcrumbs: String =
                state
                    .history
                    .iter()
                    .fold(String::new(), |mut output, folder| {
                        use std::fmt::Write;
                        _ = write!(output, "{}/", folder_name(&folder.folder.name));
                        output
                    });

            Paragraph::new(
                Line::from(vec![
                    crate::icons::strings::FOLDER_OPEN.into(),
                    " ".into(),
                    breadcrumbs.into(),
                    folder_name(&state.current_folder.name).to_string().into(),
                ])
                .bold()
                .centered(),
            )
        };

        let text_input = Paragraph::new(state.lock().search_term.clone().cyan())
            .block(Block::bordered().title("Fuzzy Search"))
            .style(Style::default().fg(Color::White));

        let items = state.lock().current_folder.list_items();

        let list = List::new(items.iter().map(|t| {
            Line::from({
                let mut spans = vec![
                t.get_icon().into(),
                " ".into(),
            ];

let indices =              SkimMatcherV2::default()
                .fuzzy_indices(t.name(), state.lock().search_term.as_str()).unwrap_or_default();

            spans.extend(indices_template(t, &indices.1));

            spans
        }).add_modifier(Modifier::DIM)
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
    }
}

fn indices_template<'a>(template: &Item, indices: &[usize]) -> Vec<Span<'a>> {
    let template_name = template.name();
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
