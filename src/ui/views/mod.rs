use ratatui::{layout::Rect, Frame};
use crate::{app::App, config::Config};

pub trait RenderableView {
    fn render(&self, frame: &mut Frame, area: Rect, config: Config, app: App) -> ();
}

pub mod file_nav;
pub mod status_bar;
pub mod header;