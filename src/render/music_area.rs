use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListDirection, ListItem};
use crate::Jade;
use crate::render::generate_list;

pub fn render_song_list(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    let list = generate_list(&jade.songs, area);
    frame.render_stateful_widget(list, area, &mut jade.song_current_selection);
}
pub fn render_time_list(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    let list: Vec<ListItem> = jade.visual_lengths
        .iter()
        .map(|length| {
            let length = length.to_string();
            ListItem::new(length)
        })
        .collect();

    let time_list = List::new(list)
        .direction(ListDirection::TopToBottom);
    frame.render_widget(time_list, area)
}