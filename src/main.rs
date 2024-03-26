use std::{io::{self, stdout}, time::Duration};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use state::{AppState, PopupState};
use ratatui::prelude::*;

use crate::config::Config;

mod views {
    pub mod header;
    pub mod status_bar;
}
mod state;
mod config;

//App const and static
const _APP_NAME: &str = "mylodon";
const _APP_VERSION: &str = "0.0.1";
const CFG: Config = config::get_config();

fn main()-> io::Result<()> {
    //App setup
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    stdout().execute(EnterAlternateScreen)?;

    //App state
    let mut app_state = AppState::Greeting;
    let mut popup_state = PopupState::None;
    let mut buffers: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    terminal.clear()?;
    let mut should_quit = false;

    //Main loop
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
        views::header::render(frame, main_layout[0], &CFG.header_bar, _APP_NAME, _APP_VERSION);

        //Body
        // let body_layout = Layout::new(
        //     Direction::Horizontal,
        //     [
        //         Constraint::Min(0),
        //     ]
        // ).split(main_layout[1]);

        // //Main body
        // match state {
        //     AppState::Greeting => {
                
        //     }
        // }

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