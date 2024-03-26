use ratatui::Frame;
use crate::views;

pub enum ViewType {
    Intro,
    TitleBar,
    StatusBar,
    Main,
    Terminal,
    Explorer,
    Search,
    Popup,
}

pub struct View {
    pub viewType: ViewType,
    pub value: Option<Vec<String>>,
    pub visible: bool,
}

impl View {
    pub fn new(viewType: ViewType, value: Option<Vec<String>>, visible: bool) -> Self {
        View {
            viewType,
            value,
            visible,
        }
    }

    pub fn render(self, frame: &mut Frame) {
        match self.viewType {
            ViewType::Intro => {
                views::intro::render(self, frame);
            }
            ViewType::TitleBar => {
                println!("TitleBar view");
            }
            ViewType::StatusBar => {
                println!("StatusBar view");
            }
            ViewType::Main => {
                println!("Main view");
            }
            ViewType::Terminal => {
                println!("Terminal view");
            }
            ViewType::Explorer => {
                println!("Explorer view");
            }
            ViewType::Search => {
                println!("Search view");
            }
            ViewType::Popup => {
                println!("Popup view");
            }
        }
    }
}