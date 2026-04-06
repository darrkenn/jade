use std::{cell::RefCell, rc::Rc};

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier},
    widgets::{List, ListState},
};

use crate::app::music::tree::Node;

pub fn render_songs(
    frame: &mut Frame,
    area: Rect,
    children: Option<&Vec<Rc<RefCell<Node>>>>,
    list_state: &mut ListState,
) {
    let mut names: Vec<String> = Vec::new();
    if let Some(children) = children {
        for child in children {
            names.push(child.borrow().name.clone());
        }
    }

    let list = List::new(names)
        .style(Color::White)
        .highlight_style(Color::Cyan);
    frame.render_stateful_widget(list, area, list_state);
}
