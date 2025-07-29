use ratatui::Frame;
use ratatui::layout::Rect;
use crate::Jade;
use crate::render::generate_list;

pub fn render_queue_list(jade: &mut Jade, area: Rect, frame: &mut Frame) {
    let list = generate_list(&jade.queue, area);
    frame.render_stateful_widget(list, area, &mut jade.queue_current_selection)
}