// mod progress_bar;
mod config;
mod copier;
mod file_to_copy;
mod progress_bar;
#[cfg(test)]
mod test;

use crate::file_to_copy::FileToCopy;
use std::{env, process, time::Instant};
fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let start = Instant::now();
    for source_path in config.source_paths {
        let mut folder = FileToCopy::from_path(&source_path);
        folder.load_files_from_path();
        folder.copy_to(&config.target_path);
    }

    let duration = start.elapsed();
    println!();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
