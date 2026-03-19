mod player;
mod songs;
use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::Style,
    text::Span,
    widgets::Block,
};

use crate::app::state::{AppState, Screen};

pub fn render(frame: &mut Frame, app_state: &mut AppState) {
    let percent_off = |v: u16, p: u16| (v * p) / 10;

    let screen_width = percent_off(frame.area().width, 9);
    let screen_x = (frame.area().width - screen_width) / 2;
    let screen_area = Rect::new(
        screen_x,
        frame.area().y,
        screen_width,
        percent_off(frame.area().height, 7),
    );

    let player_area = Rect::new(
        screen_x,
        // Below screen_area
        screen_area.height,
        screen_width,
        percent_off(screen_area.height, 3),
    );

    let screen_block = Block::bordered();
    frame.render_widget(Block::bordered().title("a"), screen_area);
    // Player area block
    frame.render_widget(Block::bordered().title("player"), player_area);

    match app_state.current.screen {
        Screen::Songs => {
            frame.render_widget(
                screen_block_selection(
                    &app_state.current.screen,
                    screen_block,
                    app_state.config.theme().text_highlight_style(),
                ),
                screen_area,
            );
            frame.render_widget(
                format!("{}", app_state.current.index),
                screen_area.inner(Margin::new(1, 1)),
            );
        }
        Screen::Queue => {
            frame.render_widget(
                screen_block_selection(
                    &app_state.current.screen,
                    screen_block,
                    app_state.config.theme().text_highlight_style(),
                ),
                screen_area,
            );
            frame.render_widget("queue", frame.area());
        }
        Screen::Settings => {
            frame.render_widget(
                screen_block_selection(
                    &app_state.current.screen,
                    screen_block,
                    app_state.config.theme().text_highlight_style(),
                ),
                screen_area,
            );
            frame.render_widget("settings", frame.area());
        }
    }
}

fn screen_block_selection<'a>(screen: &Screen, block: Block<'a>, style: Style) -> Block<'a> {
    match screen {
        Screen::Songs => block
            .title("(Esc)Settings")
            .title(Span::styled("(s)Songs", style))
            .title("(q)Queue"),
        Screen::Queue => block
            .title("(Esc)Settings")
            .title("(s)Songs")
            .title(Span::styled("(q)Queue", style)),
        Screen::Settings => block
            .title(Span::styled("(Esc)Settings", style))
            .title("(s)Songs")
            .title("(q)Queue"),
    }
}
