use std::io;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use super::{
    app::App,
    CurrentScreen,
    CurrentlyEditing
};

pub fn handle_key_event(key: KeyEvent, app: &mut App)-> Option<io::Result<bool>> {
    match app.current_screen {
        CurrentScreen::Main => match key.code {
            KeyCode::Char('e') => {
                app.current_screen = CurrentScreen::Editing;
                app.currently_editing = Some(CurrentlyEditing::Key);
            }
            KeyCode::Char('q') => {
                app.current_screen = CurrentScreen::Exiting;
            }
            _ => {}
        },
        CurrentScreen::Exiting => match key.code {
            KeyCode::Char('y') => {
                return Some(Ok(true));
            }
            KeyCode::Char('n') | KeyCode::Char('q') => {
                return Some(Ok(false));
            }
            _ => {}
        },
        CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
            match key.code {
                KeyCode::Enter => {
                    if let Some(editing) = &app.currently_editing {
                        match editing {
                            CurrentlyEditing::Key => {
                                app.currently_editing =
                                    Some(CurrentlyEditing::Value);
                            }
                            CurrentlyEditing::Value => {
                                app.save_key_value();
                                app.current_screen =
                                    CurrentScreen::Main;
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    if let Some(editing) = &app.currently_editing {
                        match editing {
                            CurrentlyEditing::Key => {
                                app.key_input.pop();
                            }
                            CurrentlyEditing::Value => {
                                app.value_input.pop();
                            }
                        }
                    }
                }
                KeyCode::Esc => {
                    app.current_screen = CurrentScreen::Main;
                    app.currently_editing = None;
                }
                KeyCode::Tab => {
                    app.toggle_editing();
                }
                KeyCode::Char(value) => {
                    if let Some(editing) = &app.currently_editing {
                        match editing {
                            CurrentlyEditing::Key => {
                                app.key_input.push(value);
                            }
                            CurrentlyEditing::Value => {
                                app.value_input.push(value);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    None
}