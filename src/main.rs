mod run;
mod render;
use color_eyre::eyre::Result;
use crate::run::run;

fn main() -> Result<()> {
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    result
}
