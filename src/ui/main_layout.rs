use std::rc::Rc;
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Text, widgets::{Block, Borders, Paragraph}, Frame};
use crate::{config, CFG};

pub fn render(f: &mut Frame) -> (Rc<[Rect]>, i8) {
    // Create the layout sections
    let mut constraints = vec![
        Constraint::Fill(1),
        Constraint::Length(1),
    ];
    let mut body_idx: i8 = 0;

    if matches!(CFG.header_bar.enabled, config::ConditionallyEnabled::Enabled) || (matches!(CFG.header_bar.enabled, config::ConditionallyEnabled::HeightBased) && f.size().height >= CFG.header_bar.height_cutoff) {
        constraints.insert(0, Constraint::Length(1));
        body_idx = 1;
    }

    let main_layout = Layout::new( Direction::Vertical, constraints).split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, *main_layout.get(0).expect("Error inserting title block into main layout: layout does not have enough sections."));

    (main_layout, body_idx)
}