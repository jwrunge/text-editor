use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use defs::View;
use state::AppState;
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

//App constants
static _APP_NAME: &str = "mylodon";
static _APP_VERSION: &str = "0.0.1";

fn main()-> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app_state = AppState::Greeting;
    let mut buffers: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut views: Vec<defs::View> = vec![
        defs::View::new(defs::ViewType::Intro, Some(vec![String::from("New header"), String::from("Body")]), true),
    ];
    const cfg: Config = config::get_config();

    let mut should_quit = false;
    while !should_quit {
        should_quit = handle_events(&mut app_state, &mut views, &mut buffers, cfg)?;
        terminal.draw(ui(views.clone(), cfg))?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(app_state: &mut AppState, views: &mut Vec<View>, buffers: &mut std::collections::HashMap<String, Vec<String>>, config: Config)-> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn ui(views: Vec<View>, config: Config)-> impl Fn(&mut Frame) {
    move |frame: &mut Frame| {
        //Header
        if config.header_bar.enabled {
            views::header::render(frame);
        }

        //Body and popups
        for view in views.clone() {
            if view.visible {
                view.render(frame);
            }
        }

        //Footer
        if config.status_bar.enabled {
            views::status_bar::render(frame);
        }
    }
}