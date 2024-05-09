use ratatui::{layout::Rect, style::{Color, Style, Stylize}, widgets::{block::Title, Block}, Frame};

use crate::config::HeaderBarConfig;

pub fn render(frame: &mut Frame, area: Rect, config: &HeaderBarConfig, name: &str, version: &str) {
    match config.enabled {
        crate::config::ConditionallyEnabled::Disabled => return,
        crate::config::ConditionallyEnabled::HeightBased => {
            if frame.size().height < config.height_cutoff {
                return;
            }
        }
        crate::config::ConditionallyEnabled::Enabled => {}
    }

    let header = Block::new()
        .style(
            Style::new()
                .bg(Color::DarkGray)
                .fg(Color::Black)
                .bold()
        );
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
