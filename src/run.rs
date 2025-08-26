use crate::app::{App, Song};
use crate::keyhandling::handle_key;
use crate::render::render;
use crate::threads::info::Info;
use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use std::time::Duration;

pub fn run(mut terminal: DefaultTerminal, app: &mut App) -> color_eyre::Result<()> {
    loop {
        if app.channels.r_update.try_recv().is_ok() {
            app.queue.remove(0);
        }

        let received_ui = app
            .channels
            .r_ui
            .recv_timeout(Duration::from_millis(10))
            .ok();

        if let Some(message) = received_ui {
            match message {
                Info::Song(song) => {
                    app.current.song = song;
                }
                Info::Position(x) => {
                    if app.current.position != x {
                        app.current.position = x;
                    }
                }
                Info::Clear => {
                    app.current.song = Song::default();
                    app.current.position = 0;
                }
            }
        }

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if handle_key(key, app) {
                    break;
                }
            }
        }

        terminal.draw(|f| render::render(f, app))?;
    }
    Ok(())
}
