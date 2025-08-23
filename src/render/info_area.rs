use crate::Jade;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Gauge, Paragraph};

pub fn render_info_area(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White))
        .render(area, frame.buffer_mut());

    let chunks = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(56), Constraint::Percentage(44)],
    )
    .margin(1)
    .split(area);

    let left_area = Layout::default()
        .margin(1)
        .constraints([Constraint::Min(0)])
        .split(chunks[0])[0];
    let right_area = Layout::default()
        .margin(1)
        .constraints([Constraint::Min(0)])
        .split(chunks[1])[0];

    let left_area_chunks = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Length(4),
        ],
    )
    .split(left_area);

    song_info(
        jade.current.title.to_owned(),
        jade.current.length,
        left_area_chunks[0],
        frame,
    );

    progress_bar(
        jade.current.position,
        jade.current.length,
        left_area_chunks[1],
        frame,
    );
}

fn progress_bar(position: u32, length: u32, area: Rect, frame: &mut Frame) {
    let ratio = if length == 0 {
        0.0
    } else {
        (position as f64 / length as f64).clamp(0.0, 1.0)
    };

    let border = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White));

    frame.render_widget(border, area);

    let inner_area = Layout::default()
        .margin(1)
        .constraints([Constraint::Min(0)])
        .split(area)[0];

    let gauge = Gauge::default()
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().bg(Color::Black).fg(Color::White)),
        )
        .gauge_style(Style::default().bg(Color::Gray).fg(Color::Cyan))
        .label("")
        .ratio(ratio);
    frame.render_widget(gauge, inner_area);
}

fn song_info(title: String, length: u32, area: Rect, frame: &mut Frame) {
    let area_chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Length(1)],
    )
    .split(area);

    let title =
        Paragraph::new(format!("Title: {}", title)).style(Style::default().bold().fg(Color::White));

    let length = Paragraph::new(format!("Length: {}", visual_length(length)))
        .style(Style::default().bold().fg(Color::White));

    frame.render_widget(title, area_chunks[0]);
    frame.render_widget(length, area_chunks[1]);
}

fn visual_length(length: u32) -> String {
    if length == 0 {
        "".to_string()
    } else {
        let mins = length / 60;
        let secs = length % 60;
        if secs < 10 {
            format!("{mins}:0{secs}")
        } else {
            format!("{mins}:{secs}")
        }
    }
}
