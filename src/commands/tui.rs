use crate::tui::app::App;
use crate::tui::event;
use anyhow::Result;

pub fn execute(lang: &str) -> Result<()> {
    let app = App::new(lang)?;
    event::run(app)
}
