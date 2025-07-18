use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Widget};
use crate::JadeConfig;

pub fn render(frame: &mut Frame, jade_config: &mut JadeConfig) {
    let area = frame.area();
    let chunks = Layout::new (
        Direction::Horizontal,
        [Constraint::Percentage(60), Constraint::Percentage(40)]
    ).split(area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White))
        .render(chunks[0], frame.buffer_mut());
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(jade_config.music_location.to_string())
        .title_bottom(jade_config.volume.to_string())
        .render(chunks[1], frame.buffer_mut());

    //Area for the songs
    let [left_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(chunks[0]);
    //Area for song information
    let [right_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(chunks[0]);
}



