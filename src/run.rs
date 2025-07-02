use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use crate::keyhandling::handle_key;
use crate::render::render;

pub fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| render(f))?;
        //Event reading
        if let Event::Key(key) = event::read()? {
            
            if handle_key(key) {
                break
            }
            
        }
    }
    Ok(())
}