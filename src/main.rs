use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use defs::View;
use state::{AppState, PopupState};
use ratatui::prelude::*;

use crate::config::Config;

mod views {
    pub mod intro;
    pub mod header;
    pub mod status_bar;
}
mod defs;
mod state;
mod config;

//App const and static
const _APP_NAME: &str = "mylodon";
const _APP_VERSION: &str = "0.0.1";
const CFG: Config = config::get_config();

fn main()-> io::Result<()> {
    //App setup
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    //App state
    let mut app_state = AppState::Greeting;
    let mut popup_state = PopupState::None;
    let mut buffers: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    //Main window structure
    let mut constraints = vec![];
    if CFG.header_bar.enabled {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Min(0));
    if CFG.status_bar.enabled {
        constraints.push(Constraint::Length(1));
    }

    //Main loop
    let mut should_quit = false;
    while !should_quit {
        should_quit = handle_events(&mut app_state, &mut buffers)?;
        terminal.draw(ui(&app_state, &popup_state))?;
    }

    //Exit app
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(app_state: &mut AppState, buffers: &mut std::collections::HashMap<String, Vec<String>>)-> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn ui(state: &AppState, popup: &PopupState)-> impl Fn(&mut Frame) {
    |frame: &mut Frame| {    
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ]
        ).split(frame.size());
        
        //Header
        if CFG.header_bar.enabled {
            views::header::render(frame, main_layout[0], &CFG.header_bar, _APP_NAME, _APP_VERSION);
        }

        //Body
        let body_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ]
        ).split(main_layout[1]);

        //Main body
        match state {
            AppState::Greeting => {
                let view = View::new(defs::ViewType::Intro, None, true);
                view.render(frame, body_layout[1], &CFG);
            }
        }

        //Popups
        match popup {
            PopupState::None => {},
        }

        //Status bar
        if CFG.status_bar.enabled {
            views::status_bar::render(frame, main_layout[2], &CFG.status_bar);
        }
    }
}