use ratatui::{layout::Rect, widgets::{block::Title, Block, Borders}, Frame};

use crate::config::HeaderBarConfig;

pub fn render(frame: &mut Frame, area: Rect, config: &HeaderBarConfig, name: &str, version: &str) {
    if config.enabled == false {
        return;
    }
    
    let header = Block::new()
        .borders(Borders::TOP);

    let header = match config.show_title {
        true => header.title(Title::from(format!("{}", name)).alignment(config.title_alignment)),
        false => header,
    };

    let header = match config.show_version {
        true => header.title(Title::from(format!("v{}", version)).alignment(config.version_alignment)),
        false => header,
    };

    let header = match &config.custom_header_text {
        Some(h) => header.title(Title::from(format!("{}", h)).alignment(config.custom_header_alignment)),
        None => header,
    };

    frame.render_widget(header, area);
}
