use std::sync::mpsc::Sender;
use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use crate::{Jade};
use crate::keyhandling::handle_key;
use crate::musicplayer::MusicPlayer;
use crate::queue::VisualQueue;
use crate::render::render;

pub fn run(mut terminal: DefaultTerminal, jade: &mut Jade, mp: Sender<MusicPlayer>, vq: Sender<VisualQueue>) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| render(f, jade))?;

        
        //Event reading
        if let Event::Key(key) = event::read()? {
            if handle_key(key, jade, mp.clone(), vq.clone()) {
                break
            }
        }
    }
    Ok(())
}