use ratatui::{widgets::{Block, Borders, Paragraph}, Frame};

pub fn render(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(String::from("Howdy"))
        .block(Block::default()
            .title(String::from("Howdy!!"))
            .borders(Borders::ALL)
        ),
        frame.size(),
    );
}
