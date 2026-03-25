use crate::tui::app::App;
use crate::tui::event;
use anyhow::Result;

pub fn execute() -> Result<()> {
    let app = App::new()?;
    event::run(app)
}
