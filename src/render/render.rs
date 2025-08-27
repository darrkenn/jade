use crate::App;
use crate::app::FocusArea;
use crate::render::info_area::render_info_area;
use crate::render::music_area::render_music_area;
use crate::render::player_area::render_player_area;
use crate::render::queue_area::render_queue_area;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;

pub fn render(frame: &mut Frame, app: &mut App) {
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

    if app.focus_area == FocusArea::Info {
        let popup_area = popup_area(area, 25, 50);
        render_info_area(app, popup_area, frame);
    } else {
        render_music_area(app, top_chunks[0], frame);
        render_queue_area(app, top_chunks[1], frame);
        render_player_area(app, vertical_chunks[1], frame);
    }
}

fn popup_area(area: Rect, x: u16, y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - y) / 2),
            Constraint::Percentage(y),
            Constraint::Percentage((100 - y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - x) / 2),
            Constraint::Percentage(x),
            Constraint::Percentage((100 - x) / 2),
        ])
        .split(popup_layout[1])[1]
}
