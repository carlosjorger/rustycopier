use std::path::PathBuf;

use clap::Parser;
#[derive(Parser)]
pub struct Config {
    pub source_paths: Vec<String>,
    pub target_path: PathBuf,
}
