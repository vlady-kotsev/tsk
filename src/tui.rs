use color_eyre::Result;
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::{
    io::{self, stdout},
    panic,
};

use crate::app::App;

fn init_terminal() -> Result<Terminal<impl Backend<Error = io::Error>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout()
            .execute(LeaveAlternateScreen)
            .expect("failed to execute LeaveAlternateScreen");
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

pub fn run_tui() -> Result<()> {
    install_panic_hook();
    let mut terminal = init_terminal()?;

    let status = App::new().and_then(|mut app| app.run(&mut terminal));

    restore_terminal()?;
    status
}
