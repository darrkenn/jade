use std::{cell::RefCell, rc::Rc};

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier},
    widgets::{List, ListState},
};

use crate::app::music::tree::{Node, NodeType};

pub fn render_songs(
    frame: &mut Frame,
    area: Rect,
    children: Option<&Vec<Rc<RefCell<Node>>>>,
    list_state: &mut ListState,
) {
    let count = if let Some(children) = children {
        children.len()
    } else {
        0
    };

    let mut names: Vec<String> = Vec::with_capacity(count);

    if let Some(children) = children {
        if count != 0 {
            for child in children {
                let name = child.borrow().name.clone();
                let formatted_name = match child.borrow().node_type {
                    NodeType::Folder => format!(" {}", name),
                    NodeType::File => format!("󰝚 {}", name),
                };
                names.push(formatted_name);
            }
        }
    }

    let list = List::new(names)
        .style(Color::White)
        .highlight_style(Color::Cyan);
    frame.render_stateful_widget(list, area, list_state);
}
