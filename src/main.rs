use color_eyre::eyre::Result;
use tsk::{app::App, tui};

fn main() -> Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let mut app = App::new();
    let status = app.run(&mut terminal);

    tui::restore_terminal()?;

    status
}
