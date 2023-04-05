
use std::fs::File;
use std::path::Path;
use std::{env, time::Instant};
use std::io::{BufWriter, Write, BufReader, BufRead, stdout, Stdout};

use crossterm::{QueueableCommand, cursor,terminal};
struct Progress{
  total_size:usize,
  consumed:usize,
  consumed_ten_scale:usize,
  stdout:Stdout
}
impl Progress {
  fn from_total_size(total_size:usize)->Self{
      Self{
        total_size,
        consumed:0,
        consumed_ten_scale:0,
        stdout:stdout()
      }
  }
  fn percent_of_consume(&self)->f64{
    (self.consumed as f64)/(self.total_size as f64)
  }
  fn percent_of_progress(&self)->f64{
    (self.consumed_ten_scale as f64)/10.0
  }
  fn consume(&mut self,lenght:usize){
    self.consumed+=lenght;
    if self.percent_of_consume()>self.percent_of_progress(){
        self.consumed_ten_scale+=1;
        let column_position=(self.consumed_ten_scale*2) as u16;
        self.stdout.queue(cursor::MoveToColumn(column_position)).unwrap();
        print!("# ");
    }
    self.stdout.queue(cursor::MoveToColumn(22)).unwrap()
                .queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    
    print!("{}/100",(self.percent_of_consume() as f32)*100.0);
    self.stdout.flush().unwrap();

  }
}
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

  let mut stream =BufWriter::with_capacity(capacity, &destiny_file);
  let mut reader=BufReader::with_capacity(capacity, &source_file);

  let size= source_file.metadata().unwrap().len() as usize;
  let mut progress=Progress::from_total_size(size);
  println!("");
  loop {
      let buffer=reader.fill_buf().expect("error in the buffer");
      let lenght= buffer.len();
      stream.write(buffer).expect("error to write");
      reader.consume(lenght);
      progress.consume(lenght);
      if lenght==0 {
          break;
      }
  }
  println!()

}

