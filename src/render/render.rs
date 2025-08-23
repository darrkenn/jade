use crate::Jade;
use crate::render::info_area::render_info_area;
use crate::render::music_area::render_music_area;
use crate::render::queue_area::render_queue_area;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;

pub fn render(frame: &mut Frame, jade: &mut Jade) {
    let area = frame.area();

    let vertical_chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(70), Constraint::Percentage(30)],
    )
    .split(area);

    let top_chunks = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(55), Constraint::Percentage(45)],
    )
    .split(vertical_chunks[0]);

    render_music_area(jade, top_chunks[0], frame);
    render_queue_area(jade, top_chunks[1], frame);
    render_info_area(jade, vertical_chunks[1], frame);
}
