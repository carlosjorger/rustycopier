
use std::fs::File;
use std::path::Path;
use std::{env, time::Instant};
use std::io::{BufWriter, Write, BufReader, BufRead};
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()<=1 {
      println!("Please pass the origin path");
      return;
    }
    if args.len()<=2 {
      println!("Please pass the destiny path");
      return;
    }
    let source=&args[1];
    let destiny=&args[2];
    let file_name = &get_file_name_from_path(source);
    let destiny_with_file=add_file_name_to_path(destiny, file_name);
    let start = Instant::now();
    copy(&source, &destiny_with_file);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
fn get_file_name_from_path(path:&str)->String{
  let path= Path::new(&path);
  let file_name = path.file_name()
                    .expect("the origin path is not a valid file")
                    .to_str().unwrap();
  file_name.to_string()
}
fn add_file_name_to_path(path:&str,file_name:&str)->String{
  let path=Path::new(&path);
  let path_with_file_name=path.join(file_name);
  let path_str=path_with_file_name.to_str().unwrap();
  path_str.to_string()
}
fn copy(source_path:&str,destiny_path:&str ) {
    let file = File::create(destiny_path)
    .expect("Should have been able to read the destiny path");
    let mut stream =BufWriter::with_capacity(1024*1000, file);
    let file = File::open(source_path)
    .expect("Should have been able to read the source path");
    let mut reader=BufReader::with_capacity(1024*1000, file);
    loop {
        let buffer=reader.fill_buf().expect("error in the buffer");
        stream.write(buffer).expect("error to write");
        let lenght= buffer.len();
        reader.consume(lenght);
        if lenght==0 {
            break;
        }
    }
}

