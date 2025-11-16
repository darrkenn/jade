use edar::{FormatDuration, Metadata};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_image::{Image, StatefulImage};

pub fn render_info_area(metadata: Metadata, area: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));

    let [inner_area] = Layout::new(Direction::Horizontal, [Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    let lines: Vec<Line> = [
        ("Title", metadata.title.unwrap_or("".to_string())),
        ("Artist", metadata.artist.unwrap_or("".to_string())),
        ("Album", metadata.album.unwrap_or("".to_string())),
        ("Genre", metadata.genre.unwrap_or("".to_string())),
        ("Duration", metadata.duration.format()),
    ]
    .iter()
    .map(|(label, value)| Line::from(format!("{:<10} {value}", label)))
    .collect();

    let info = Paragraph::new(lines).style(Style::default().fg(Color::White));

    frame.render_widget(block, area);
    frame.render_widget(info, inner_area);
}
