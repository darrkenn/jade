use crate::app::{App, FocusArea::Music as Music_Area};
use crate::render::generate_list::generate_list;
use edar::FormatDuration;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::layout::{HorizontalAlignment::Center, Rect};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, List, ListDirection, ListItem};

pub fn render_music_area(app: &mut App, area: Rect, frame: &mut Frame) {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Music")
        .title_alignment(Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .border_style(if app.focus_area == Music_Area {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        })
        .render(area, frame.buffer_mut());

    let [left_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    //Splits songs area into title+time
    let left_area_chunks = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(90), Constraint::Percentage(10)],
    )
    .split(left_inner_area);

    render_song_list(app, left_area_chunks[0], frame);
    render_time_list(app, left_area_chunks[1], frame);
}

pub fn render_song_list(app: &mut App, area: Rect, frame: &mut Frame) {
    let values: Vec<String> = app
        .songs
        .iter()
        .map(|s| {
            if let Some(title) = s.metadata.title.clone() {
                title
            } else {
                let mut split: Vec<&str> = s.file_name.split(".").collect();
                if !split.is_empty() {
                    split.pop();
                }
                split.join(".").to_string()
            }
        })
        .collect();

    let list = generate_list(&values, area);

    frame.render_stateful_widget(list, area, &mut app.current.selection.song);
}
pub fn render_time_list(app: &mut App, area: Rect, frame: &mut Frame) {
    let values: Vec<ListItem> = app
        .songs
        .iter()
        .map(|s| {
            if let Some(length) = Some(s.metadata.duration.format()) {
                ListItem::new(length)
            } else {
                ListItem::from("00:00")
            }
        })
        .collect();

    let time_list = List::new(values).direction(ListDirection::TopToBottom);
    frame.render_widget(time_list, area)
}
