mod config;
mod copier;
mod copier_pool;
mod progress_counter;
#[cfg(test)]
mod test;
use std::time::Instant;

use clap::Parser;

use crate::copier::FileToCopy;
// TODO: apply this doc https://rust-cli.github.io/book/index.html
fn main() {
    let config = config::Config::parse();
    let start = Instant::now();
    for source_path in config.source_paths {
        let mut folder = FileToCopy::from_path(&source_path);
        folder.load_files_from_path();
        folder.copy_to(config.target_path.as_path());
    }
    let duration = start.elapsed();
    println!();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
