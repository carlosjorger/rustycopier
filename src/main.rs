// mod progress_bar;
mod config;
mod copier;
mod folder;
mod progress_bar;
use crate::folder::Folder;
use std::{env, process, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let mut folder = Folder::from_path(&config.source_path);
    folder.load_files_from_path();
    let start = Instant::now();
    folder.copy_to(&config.target_path);
    let duration = start.elapsed();
    println!();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
