use crate::app::{App, FocusArea::Queue as Queue_Area};
use crate::render::generate_list::generate_list;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::{HorizontalAlignment::Center, Rect};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType};

pub fn render_queue_area(app: &mut App, area: Rect, frame: &mut Frame) {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Queue")
        .title_alignment(Center)
        .border_style(if app.focus_area == Queue_Area {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        })
        .render(area, frame.buffer_mut());

    let [right_inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);
    render_queue_list(app, right_inner_area, frame);
}

fn render_queue_list(app: &mut App, area: Rect, frame: &mut Frame) {
    let values: Vec<String> = app
        .queue
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
    frame.render_stateful_widget(list, area, &mut app.queue_current_selection)
}
