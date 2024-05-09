use std::rc::Rc;
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, Frame};
use crate::{config, CFG};

pub fn assemble(f: &mut Frame) -> (Rc<[Rect]>, usize) {
    // Create the layout sections
    let mut constraints = vec![
        Constraint::Fill(1),
        Constraint::Length(1),
    ];
    let mut body_idx: usize = 0;

    if matches!(CFG.header_bar.enabled, config::ConditionallyEnabled::Enabled) || (matches!(CFG.header_bar.enabled, config::ConditionallyEnabled::HeightBased) && f.size().height >= CFG.header_bar.height_cutoff) {
        constraints.insert(0, Constraint::Length(1));
        body_idx = 1;
    }

    let main_layout = Layout::new( Direction::Vertical, constraints).split(f.size());
    (main_layout, body_idx)
}