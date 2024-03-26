use std::{io::{self, stdout}, time::Duration};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui_splash_screen::{SplashConfig, SplashScreen};
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

//Splash
static SPLASH_CONFIG: SplashConfig = SplashConfig {
    image_data: include_bytes!("./assets/alt.png"),
    sha256sum: None,
    render_steps: 12,
    use_colors: true
};

fn main()-> io::Result<()> {
    //App setup
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    //App state
    let mut app_state = AppState::Greeting;
    let mut popup_state = PopupState::None;
    let mut buffers: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut splash: SplashScreen = match SplashScreen::new(SPLASH_CONFIG) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    //Main window structure
    let mut constraints = vec![];
    if CFG.header_bar.enabled {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Min(0));
    if CFG.status_bar.enabled {
        constraints.push(Constraint::Length(1));
    }

    terminal.clear()?;
    let mut should_quit = false;

    //Main loop
    while !should_quit {
        should_quit = handle_events(&mut app_state, &mut buffers)?;

        if splash.is_rendered() {
            terminal.draw(ui(&app_state, &popup_state))?;
        }
        else {
            terminal.draw(|frame| {
                frame.render_widget(&mut splash, frame.size());
            })?;
            std::thread::sleep(Duration::from_millis(100));

            if splash.is_rendered() {
                std::thread::sleep(Duration::from_secs(1));
                terminal.clear()?;
                stdout().execute(EnterAlternateScreen)?;
            }
        }
    }

    //Exit app
    terminal.clear()?;
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