use std::{io::stdout, sync::Mutex, thread::current};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind, KeyEventState},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use serde::de;
use style::Styled;

use crate::template::Template;

pub fn pick_template(templates: &[super::Template]) -> anyhow::Result<Option<super::Template>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let list_state = Mutex::new({
        let mut state = ListState::default();
        state.select(Some(0));
        state
    });

    let ui = |frame: &mut Frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        let list = List::new(templates.iter().map(|t| t.to_string()))
            .block(Block::bordered().title("Templates"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::LightCyan))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, chunks[0], &mut *list_state.lock().unwrap());
    };

    let selected = loop {
        terminal.draw(ui)?;
        let (should_quit, selected) = handle_events(templates, &list_state)?;

        if should_quit {
            break selected;
        }
    };

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(selected)
}

fn handle_events(
    templates: &[super::Template],
    list_state: &Mutex<ListState>,
) -> anyhow::Result<(bool, Option<Template>)> {
    let mut should_quit = false;

    let current_index = list_state.lock().unwrap().selected().unwrap_or(0);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Adjustment {
        Up,
        Down,
    }

    let adjust_index = |index: usize, adjustment: Adjustment| {
        if index == 0 && adjustment == Adjustment::Down {
            templates.len() - 1
        } else if index == templates.len() - 1 && adjustment == Adjustment::Up {
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
                KeyCode::Char('q') => should_quit = true,
                KeyCode::Up => list_state
                    .lock()
                    .unwrap()
                    .select(Some(adjust_index(current_index, Adjustment::Down))),
                KeyCode::Down => list_state
                    .lock()
                    .unwrap()
                    .select(Some(adjust_index(current_index, Adjustment::Up))),
                KeyCode::Enter => {
                    let selected = list_state.lock().unwrap().selected();

                    if let Some(selected) = selected {
                        return Ok((false, Some(templates[selected].clone())));
                    }
                }
                _ => {}
            }
        }
    }

    Ok((should_quit, None))
}
