use ratatui::{layout::Rect, widgets::{Block, Borders}, Frame};

use crate::config::StatusBarConfig;

pub fn render(frame: &mut Frame, area: Rect, config: &StatusBarConfig) {
    let header = Block::new()
        .borders(Borders::BOTTOM);

    frame.render_widget(header, area);
}
