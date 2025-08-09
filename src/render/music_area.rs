use crate::FocusArea::Music;
use crate::Jade;
use crate::render::generate_list::generate_list;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::layout::{HorizontalAlignment::Center, Rect};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, List, ListDirection, ListItem};

pub fn render_music_area(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Music")
        .title_alignment(Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .border_style(if jade.focus_area == Music {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Red)
        })
        .render(area, frame.buffer_mut());

    let [left_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    //Splits songs area into title+time
    let left_area_chunks = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(80), Constraint::Percentage(20)],
    )
    .split(left_inner_area);

    render_song_list(jade, left_area_chunks[0], frame);
    render_time_list(jade, left_area_chunks[1], frame);
}

pub fn render_song_list(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    let list = generate_list(&jade.songs.titles, area);
    frame.render_stateful_widget(list, area, &mut jade.song_current_selection);
}
pub fn render_time_list(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    let list: Vec<ListItem> = jade
        .songs
        .visual_lengths
        .iter()
        .map(|length| {
            let length = length.to_string();
            ListItem::new(length)
        })
        .collect();

    let time_list = List::new(list).direction(ListDirection::TopToBottom);
    frame.render_widget(time_list, area)
}
