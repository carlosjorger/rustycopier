mod config;
mod copier;
mod copier_pool;
mod progress_counter;
mod utils;
use crate::copier::FileToCopy;
use clap::Parser;
use std::time::Instant;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::parse();
    let start = Instant::now();

    if let Some((target_path, source_paths)) = config.paths.split_last() {
        for source_path in source_paths {
            let mut folder = FileToCopy::from_path(source_path);
            folder.load_files_from_path()?;
            folder.copy_to(target_path, config.disable_logging);
        }
    }
    let duration = start.elapsed();

    println!();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}
