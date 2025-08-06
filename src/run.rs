use crate::Jade;
use crate::keyhandling::handle_key;
use crate::musicplayer::{MusicPlayer, Queue};
use crate::render::render;
use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use std::sync::mpsc::Sender;

pub fn run(
    mut terminal: DefaultTerminal,
    jade: &mut Jade,
    mp: Sender<MusicPlayer>,
    q: Sender<Queue>,
) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| render(f, jade))?;

        //Event reading
        if let Event::Key(key) = event::read()? {
            if handle_key(key, jade, mp.clone(), q.clone()) {
                break;
            }
        }
    }
    Ok(())
}
