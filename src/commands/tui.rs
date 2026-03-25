use crate::tui::app::App;
use crate::tui::event;
use anyhow::Result;

pub fn execute(lang: &str) -> Result<()> {
    // Если язык не указан или "default", читаем из конфига
    let lang = if lang == "en" || lang.is_empty() {
        get_default_lang()
    } else {
        lang.to_string()
    };

    let app = App::new(&lang)?;
    event::run(app)
}

fn get_default_lang() -> String {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
        .join("todo-rs");
    let config_file = config_dir.join("config");

    if config_file.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_file) {
            for line in content.lines() {
                if line.starts_with("lang=") {
                    return line.trim_start_matches("lang=").trim().to_string();
                }
            }
        }
    }
    "en".to_string()
}
