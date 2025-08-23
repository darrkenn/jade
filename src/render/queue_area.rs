use crate::jade::{FocusArea::Queue as Queue_Area, Jade};
use crate::render::generate_list::generate_list;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::{HorizontalAlignment::Center, Rect};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType};

pub fn render_queue_area(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Queue")
        .title_alignment(Center)
        .border_style(if jade.focus_area == Queue_Area {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        })
        .render(area, frame.buffer_mut());

    let [right_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);
    render_queue_list(jade, right_inner_area, frame);
}

fn render_queue_list(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    let list = generate_list(
        &jade.queue,
        area,
        jade.config.music_location.to_str().unwrap(),
    );
    frame.render_stateful_widget(list, area, &mut jade.queue_current_selection)
}
