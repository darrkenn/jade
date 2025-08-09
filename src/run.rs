use crate::Jade;
use crate::keyhandling::handle_key;
use crate::render::render;
use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use std::time::Duration;

pub fn run(mut terminal: DefaultTerminal, jade: &mut Jade) -> color_eyre::Result<()> {
    loop {
        if jade.channels.r_update.try_recv().is_ok() {
            jade.queue.remove(0);
        }
        terminal.draw(|f| render(f, jade))?;

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
