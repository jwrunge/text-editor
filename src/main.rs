use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

mod views {
    pub mod intro;
}

mod defs;

fn main()-> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let views: Vec<defs::View> = vec![
        defs::View::new(defs::ViewType::Intro, None, true),
    ];

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events()-> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn ui(frame: &mut Frame) {

    // frame.render_widget(
    //     views::intro::render(),
    //     frame.size(),
    // );
    // let main_layout = Layout::new(
    //     Direction::Vertical,
    //     vec![
    //         Constraint::Length(1),
    //         Constraint::Min(0),
    //         Constraint::Length(1),
    //     ],
    // )
    // .split(frame.size());

    // frame.render_widget(
    //     Block::new().borders(Borders::TOP).title("Title bar"),
    //     main_layout[0],
    // );

    // frame.render_widget(
    //     Block::new().borders(Borders::TOP).title("Status bar"),
    //     main_layout[2],
    // );

    // let inner_layout = Layout::new(
    //     Direction::Horizontal,
    //     vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    // )
    // .split(main_layout[1]);

    // frame.render_widget(
    //     Block::default().borders(Borders::ALL).title("Left"),
    //     inner_layout[0],
    // );
    // frame.render_widget(
    //     Block::default().borders(Borders::ALL).title("Right"),
    //     inner_layout[1],
    // );
}