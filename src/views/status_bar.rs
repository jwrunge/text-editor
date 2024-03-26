use ratatui::{widgets::{Block, Borders, Paragraph}, Frame};

pub fn render(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(String::from("YO"))
        .block(Block::default()
            .title(String::from("YO!!"))
            .borders(Borders::ALL)
        ),
        frame.size(),
    );
}
