use ratatui::{widgets::{Block, Borders, Paragraph}, Frame};

use crate::defs::View;

pub fn render(view: View, frame: &mut Frame) {
    match view.value {
        Some(value) => {
            if(value.len() != 2) { return; }

            frame.render_widget(
                Paragraph::new(value[1].to_owned())
                .block(Block::default()
                    .title(value[0].to_owned())
                    .borders(Borders::ALL)
                ),
                frame.size(),
            );
        },
        None => {
            return;
        }
    
    }
}
