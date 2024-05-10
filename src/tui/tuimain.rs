
// TUI Deps
use ratatui::{prelude::*, widgets::*};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand
};
use std::io::{self, stdout};

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        // Exit key
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("» Omniscape (© 2001 - 2024 NWO-IT GmbH) « "),
        main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Q » Quit"),
        main_layout[2],
    );
    let inner_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]
    ).split(main_layout[1]);

    frame.render_widget(
        Block::bordered().title(" Database Statistics "),
        inner_layout[0]
    );
    frame.render_widget(
        Block::bordered().title(" Task » TikTok Scrape "),
        inner_layout[1]
    );
}