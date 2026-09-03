use color_eyre::{Result, eyre::eyre};
use copypasta::{ClipboardContext, ClipboardProvider};

pub struct Clipboard {
    ctx: ClipboardContext,
}

impl Clipboard {
    pub fn new() -> Result<Self> {
        let ctx =
            ClipboardContext::new().map_err(|_| eyre!("clipboard: can't create clipboard ctx"))?;

        Ok(Self { ctx })
    }

    pub fn copy_to_clipboard(&mut self, data: &str) -> Result<()> {
        self.ctx
            .set_contents(data.to_string())
            .map_err(|_| eyre!("clipboard: failed to copy into clipbaord"))?;

        Ok(())
    }
}
