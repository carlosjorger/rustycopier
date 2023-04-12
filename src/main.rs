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

// fn copy(source_path: &str, destiny_path: &str, capacity: usize) {
//     let destiny_file =
//         File::create(destiny_path).expect("Should have been able to read the destiny path");
//     let source_file =
//         File::open(source_path).expect("Should have been able to read the source path");

//     let mut stream = BufWriter::with_capacity(capacity, &destiny_file);
//     let mut reader = BufReader::with_capacity(capacity, &source_file);

//     let size = source_file.metadata().unwrap().len() as usize;
//     let mut progress = progress_bar::ProgressBar::from_total_size(size);
//     println!();
//     loop {
//         let buffer = reader.fill_buf().expect("error in the buffer");
//         let lenght = buffer.len();
//         stream.write_all(buffer).expect("error to write");
//         reader.consume(lenght);
//         progress.consume(lenght);
//         if lenght == 0 {
//             break;
//         }
//     }
//     println!()
// }
