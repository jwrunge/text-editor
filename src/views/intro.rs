use ratatui::{widgets::{Block, Borders, Paragraph}, Frame};

use crate::defs::View;

pub fn render(view: View, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!!!")
        .block(Block::default()
            .title("Greeting")
            .borders(Borders::ALL)
        ),
        frame.size(),
    );
}
