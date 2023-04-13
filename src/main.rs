// mod progress_bar;
mod copier;
mod folder;
mod progress_bar;
use std::{env, time::Instant};

use crate::folder::Folder;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please pass the origin path");
        return;
    }
    if args.len() <= 2 {
        println!("Please pass the destiny path");
        return;
    }

    let source = &args[1];
    let destiny = &args[2];
    let mut folder = Folder::from_path(source);
    folder.load_files_from_path();
    let start = Instant::now();
    folder.copy_to(destiny);
    let duration = start.elapsed();
    println!();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
