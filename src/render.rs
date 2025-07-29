mod music_area;
mod queue_area;

use unicode_width::UnicodeWidthStr;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::HorizontalAlignment::Center;
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style};
use ratatui::style::Color::{Green};
use ratatui::widgets::{Block, BorderType, HighlightSpacing, List, ListDirection, ListItem, Widget};
use crate::FocusArea::{Music, Queue};
use crate::Jade;
use crate::render::music_area::{render_song_list, render_time_list};
use crate::render::queue_area::{render_queue_list};

pub fn render(frame: &mut Frame, jade: &mut Jade) {
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
        .title_top("Music")
        .title_alignment(Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .border_style(if jade.focus_area == Music {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        })
        .render(horizontal_chunks[0], frame.buffer_mut());
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Queue")
        .title_alignment(Center)
        .border_style(if jade.focus_area == Queue {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        }
        )
        .render(horizontal_chunks[1], frame.buffer_mut());


    //Area for the songs
    let [left_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(horizontal_chunks[0]);
    //Area for song information
    let [right_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(horizontal_chunks[1]);

    let left_area_chunks = Layout::new(
        Direction::Horizontal, [Constraint::Percentage(80), Constraint::Percentage(20)]
    ).split(left_inner_area);

    //Rendering of widgets
    render_song_list(jade, left_area_chunks[0], frame);
    render_time_list(jade, left_area_chunks[1], frame);
    render_queue_list(jade, right_inner_area, frame);
}

pub fn generate_list(values: &Vec<String>, area: Rect) -> List {
    let map: Vec<ListItem> = values
        .iter()
        .map(|song| {
            let max = area.width.saturating_sub(10) as usize;
            let title = if song.width() < max {
                song.to_string()
            } else {
                format!("{}...", song.chars().take(max.saturating_sub(3)).collect::<String>().as_str())
            };
            ListItem::from(title)
        }).collect();

    let list = List::new(map)
        .style(Style::new().gray())
        .highlight_style(Style::default().bold().fg(Green))
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol("-> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);
    list
}



