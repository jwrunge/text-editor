use ratatui::{layout::Rect, Frame};
use crate::{config::Config, views};

#[derive(Clone)]
pub enum ViewType {
    Intro,
    // TitleBar,
    // StatusBar,
    // Main,
    // Terminal,
    // Explorer,
    // Search,
    // Popup,
}

#[derive(Clone)]
pub struct View {
    pub view_type: ViewType,
    pub value: Option<Vec<String>>,
    pub visible: bool,
}

impl View {
    pub fn new(view_type: ViewType, value: Option<Vec<String>>, visible: bool) -> Self {
        View {
            view_type,
            value,
            visible,
        }
    }

    pub fn render(self, frame: &mut Frame, area: Rect, config: &Config) {
        match self.view_type {
            ViewType::Intro => {
                views::intro::render(self, frame, area, config);
            }
            // ViewType::TitleBar => {
            //     println!("TitleBar view");
            // }
            // ViewType::StatusBar => {
            //     println!("StatusBar view");
            // }
            // ViewType::Main => {
            //     println!("Main view");
            // }
            // ViewType::Terminal => {
            //     println!("Terminal view");
            // }
            // ViewType::Explorer => {
            //     println!("Explorer view");
            // }
            // ViewType::Search => {
            //     println!("Search view");
            // }
            // ViewType::Popup => {
            //     println!("Popup view");
            // }
        }
    }
}