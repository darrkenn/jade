use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use crate::JadeConfig;
use crate::keyhandling::handle_key;
use crate::render::render;

pub fn run(mut terminal: DefaultTerminal, jade_config: &mut JadeConfig) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| render(f, jade_config))?;
        //Event reading
        if let Event::Key(key) = event::read()? {
            if handle_key(key) {
                break
            }
        }
    }
    Ok(())
}