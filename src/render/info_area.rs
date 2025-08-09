use crate::Jade;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType};

pub fn render_info_area(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White))
        .render(area, frame.buffer_mut());
}
