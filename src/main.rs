mod config;
mod parser;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use glob::glob;
use log::{error, info};
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(version, about = "Keboola JSON to CSV Processor")]
struct Cli {
    #[arg(long, default_value = "/data")]
    data_dir: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let config_path = cli.data_dir.join("config.json");
    let config: config::Config = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?
        .pipe(|s| serde_json::from_str(&s))
        .with_context(|| "Failed to parse config file")?;

    config.validate()?;

    let input_dir = match config.parameters.in_type {
        config::InputType::Files => cli.data_dir.join("in/files"),
        config::InputType::Tables => cli.data_dir.join("in/tables"),
    };

    let output_dir = cli.data_dir.join("out/tables");
    std::fs::create_dir_all(&output_dir)?;

    let parser = parser::Parser::new(config, output_dir);

    let pattern = input_dir.join("*.json");
    let pattern_str = pattern.to_string_lossy();

    for entry in glob(&pattern_str)? {
        match entry {
            Ok(path) => {
                info!("Processing file: {}", path.display());
                if let Err(e) = parser.process_file(&path) {
                    error!("Error processing file {}: {}", path.display(), e);
                }
            }
            Err(e) => error!("Error in glob pattern: {}", e),
        }
    }

    Ok(())
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}
