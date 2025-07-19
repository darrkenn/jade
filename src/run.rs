use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use crate::Jade;
use crate::keyhandling::handle_key;
use crate::render::render;

pub fn run(mut terminal: DefaultTerminal, jade: &mut Jade, songs: Vec<String>) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| render(f, jade, songs.clone()))?;

        //Event reading
        if let Event::Key(key) = event::read()? {
            if handle_key(key, jade, songs.clone()) {
                break
            }
        }
    }
    Ok(())
}