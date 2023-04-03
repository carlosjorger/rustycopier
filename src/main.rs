
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
    copy(&source, &destiny_with_file,1024*1000);
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
fn copy(source_path:&str,destiny_path:&str,capacity:usize) {
  let destiny_file = File::create(destiny_path)
  .expect("Should have been able to read the destiny path");
  let source_file = File::open(source_path)
  .expect("Should have been able to read the source path");
  let size= source_file.metadata().unwrap().len() as usize;
  
  let mut stream =BufWriter::with_capacity(capacity, &destiny_file);
  let mut reader=BufReader::with_capacity(capacity, &source_file);
  
  let mut consumed=0;
  let mut progress=0;
  loop {
      let buffer=reader.fill_buf().expect("error in the buffer");
      let lenght= buffer.len();
      stream.write(buffer).expect("error to write");
      reader.consume(lenght);
      consumed+=lenght;
      let percent_of_consume=(consumed as f64)/(size as f64);
      let percent_of_progress=(progress as f64)/10.0;
      if percent_of_consume>percent_of_progress{
        progress+=1;
        print!("# ");
      }
      if lenght==0 {
          break;
      }
  }
  println!()

}

