mod cli;
mod commands;
mod config;
mod copy;
mod output;
mod ui;
mod utils;

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::utils::load_translations;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let translations = load_translations()?;
    let config = Config::load_or_default();

    let lang_code = if config.language == "auto" {
        sys_locale::get_locale()
            .and_then(|val| val.split(&['_', '-']).next().map(str::to_lowercase))
            .unwrap_or_else(|| "en".to_string())
    } else {
        config.language.to_lowercase()
    };
    let msg = match translations.get(&lang_code) {
        Some(messages) => messages,
        None => {
            let fallback = translations
                .get("en")
                .expect("English translations missing");
            eprintln!(
                "{}",
                fallback.language_not_supported.replace("{}", &lang_code)
            );
            fallback
        }
    };

    match &cli.command {
        Commands::Config { .. } => commands::handle_conf(&cli.command, msg, &config),
        Commands::Copy { .. } => commands::handle_copy(&cli.command, msg, &config),
    }
}
