use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::Direction;

pub fn render(frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::new (
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)]
    ).split(area);

    
}

