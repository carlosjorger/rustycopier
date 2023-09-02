mod config;
mod copier;
mod copier_pool;
mod progress_counter;
#[cfg(test)]
mod test;
use std::{path::Path, time::Instant};

use clap::Parser;

use crate::copier::FileToCopy;
// TODO: apply this doc https://rust-cli.github.io/book/index.html
fn main() {
    let config = config::Config::parse();
    let start = Instant::now();
    println!("{:#?}", config.paths);
    if let Some((target_path, source_paths)) = config.paths.split_last() {
        for source_path in source_paths {
            let mut folder = FileToCopy::from_path(source_path);
            folder.load_files_from_path();
            folder.copy_to(Path::new(target_path));
        }
    }
    let duration = start.elapsed();
    println!();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
