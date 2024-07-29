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

pub fn pick_template(templates: &[super::Template]) -> anyhow::Result<super::Template> {
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

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events(templates, &list_state)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(todo!())
}

fn handle_events(
    templates: &[super::Template],
    list_state: &Mutex<ListState>,
) -> anyhow::Result<bool> {
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
                _ => {}
            }
        }
    }

    Ok(should_quit)
}
