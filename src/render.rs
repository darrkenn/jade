use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, HighlightSpacing, List, ListDirection, Widget};
use crate::Jade;

pub fn render(frame: &mut Frame, jade: &mut Jade, songs: Vec<String>) {
    let area = frame.area();

    let vertical_chunks = Layout::new (
        Direction::Vertical, [Constraint::Percentage(70), Constraint::Percentage(30)]
    ).split(area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White))
        .render(vertical_chunks[1], frame.buffer_mut());

    let horizontal_chunks = Layout::new (
        Direction::Horizontal, [Constraint::Percentage(55), Constraint::Percentage(45)]
    ).split(vertical_chunks[0]);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White))
        .render(horizontal_chunks[0], frame.buffer_mut());
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(jade.music_location.to_string())
        .title_bottom(jade.volume.to_string())
        .render(horizontal_chunks[1], frame.buffer_mut());


    //Area for the songs
    let [left_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(horizontal_chunks[0]);
    //Area for song information
    let [right_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(horizontal_chunks[1]);

    let song_list = List::new(songs)
        .style(Style::new().gray())
        .highlight_style(Style::new().bold())
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol("-> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(song_list, left_inner_area, &mut jade.current_selection)
}





