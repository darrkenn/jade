use edar::FormatDuration;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, ListItem, Paragraph},
};

use crate::app::App;

pub fn render_info_area(app: &App, area: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White).bg(Color::Black));

    let [inner_area] = Layout::new(Direction::Horizontal, [Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    let metadata = app.song_info.metadata.clone();

    let lines: Vec<Line> = [
        ("Title", metadata.title.unwrap_or("".to_string())),
        ("Artist", metadata.artist.unwrap_or("".to_string())),
        ("Album", metadata.album.unwrap_or("".to_string())),
        ("Genre", metadata.genre.unwrap_or("".to_string())),
        ("Duration", metadata.duration.format()),
    ]
    .iter()
    .map(|(label, value)| Line::from(format!("{label}: {value}")))
    .collect();

    let info = Paragraph::new(lines);

    frame.render_widget(block, area);
    frame.render_widget(info, inner_area);
}
