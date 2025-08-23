use crate::Jade;
use crate::keyhandling::handle_key;
use crate::render::render;
use crate::threads::info::Info;
use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use std::time::Duration;

pub fn run(mut terminal: DefaultTerminal, jade: &mut Jade) -> color_eyre::Result<()> {
    loop {
        if jade.channels.r_update.try_recv().is_ok() {
            jade.queue.remove(0);
        }

        let received_ui = jade
            .channels
            .r_ui
            .recv_timeout(Duration::from_millis(10))
            .ok();

        if let Some(message) = received_ui {
            match message {
                Info::Song(song, length) => {
                    if let Some(title) = song.split("/").last() {
                        jade.current.title = title.to_string();
                    } else {
                        jade.current.title = song;
                    }
                    jade.current.length = length;
                }
                Info::Position(x) => {
                    if jade.current.position != x {
                        jade.current.position = x;
                    }
                }
                Info::Clear => {
                    jade.current.title = "".to_string();
                    jade.current.length = 0;
                    jade.current.position = 0;
                }
            }
        }

        terminal.draw(|f| render::render(f, jade))?;

        //Event reading
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if handle_key(key, jade) {
                    break;
                }
            }
        }
    }
    Ok(())
}
