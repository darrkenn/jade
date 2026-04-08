use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{music::tree::NodeType, state::AppState};

pub fn handle_songs_key(key: KeyEvent, app_state: &mut AppState) {
    match key.code {
        KeyCode::Up => app_state.list_state.select_previous(),
        KeyCode::Down => app_state.list_state.select_next(),
        KeyCode::Left => {
            // If current node has parent, set current to parent
            if let Some(node) = app_state.current.node.clone() {
                let previous_selected_index = app_state.list_state.selected();
                if let Some(parent) = &node.borrow().parent {
                    if let Some(upgraded_parent) = parent.upgrade() {
                        app_state
                            .list_state
                            .select(upgraded_parent.borrow().child_index);
                        app_state.current.node = Some(upgraded_parent);
                    }
                }
                node.borrow_mut().child_index = previous_selected_index;
            }
        }
        KeyCode::Enter | KeyCode::Right => {
            // Get current node, if current node is not set just default to root
            let node = if let Some(node) = app_state.current.node.clone() {
                node
            } else {
                app_state.root.clone()
            };

            let previous_selected_index = app_state.list_state.selected();

            if let Some(children) = &node.borrow().children {
                if let Some(selected_node) = children
                    .get(app_state.list_state.selected().unwrap_or(0).clone())
                    .cloned()
                {
                    if selected_node.borrow().node_type == NodeType::Folder {
                        let location = selected_node.borrow().get_location(&selected_node);
                        selected_node
                            .borrow_mut()
                            .explore(Some(location), &selected_node)
                            .ok();
                        selected_node.borrow_mut().sort_children();
                        app_state.list_state.select(
                            if !selected_node.borrow().child_index.is_some() {
                                Some(0)
                            } else {
                                selected_node.borrow().child_index
                            },
                        );
                        app_state.current.node = Some(selected_node);
                    }
                }
            } else {
                return;
            };
            node.borrow_mut().child_index = previous_selected_index;
        }
        _ => {}
    }
}
