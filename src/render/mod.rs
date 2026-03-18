mod player;
mod songs;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    widgets::Block,
};

use crate::app::state::{AppState, Screen};

pub fn render(frame: &mut Frame, app_state: &mut AppState) {
    let percent_off = |v: u16, p: u16| (v * p) / 10;

    let screen_width = percent_off(frame.area().width, 9);
    let screen_area = Rect::new(
        (frame.area().width - screen_width) / 2,
        frame.area().y,
        // 90% of frame.area()
        screen_width,
        // 80% of frame.area()
        percent_off(frame.area().height, 8),
    );

    frame.render_widget(Block::bordered().title("a"), screen_area);

    match app_state.current_screen {
        Screen::Songs => {
            //frame.render_widget(format!("{}", app_state.current_volume_level), frame.area());
        }
        Screen::Queue => {
            frame.render_widget("queue", frame.area());
        }
        Screen::Settings => {
            frame.render_widget("settings", frame.area());
        }
    }
}
