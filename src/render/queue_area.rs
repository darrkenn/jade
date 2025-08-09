use crate::Jade;
use crate::render::generate_list;
use ratatui::Frame;
use ratatui::layout::Rect;

pub fn render_queue_list(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    let list = generate_list(&jade.queue, area);
    frame.render_stateful_widget(list, area, &mut jade.queue_current_selection)
}

