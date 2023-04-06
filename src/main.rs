
use std::fs::File;
use std::path::Path;
use std::{env, time::Instant};
use std::io::{BufWriter, Write, BufReader, BufRead, stdout, Stdout};

use crossterm::{QueueableCommand, cursor,terminal};

struct ProgressBar{
  stdout:Stdout,
  scale:usize
}
impl ProgressBar {
    fn progress_bar(scale:usize)->Self{
      Self { stdout:stdout(),scale:scale }
    }
    fn draw_a_bar(&mut self,progress_base_ten:usize){
      let column_position=(progress_base_ten) as u16;
      self.stdout.queue(cursor::MoveToColumn(column_position)).unwrap();
      if progress_base_ten==self.scale {
        print!("=");         
      }
      else {
        print!("=>");
      }
    }
    fn change_progress_number(&mut self,fraction_of_consume:f64){
      self.stdout.queue(cursor::MoveToColumn((self.scale+2) as u16)).unwrap()
                      .queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
      let percent_of_consume=(fraction_of_consume as f32)*100.0;
      if fraction_of_consume==1.0 {
        print!("{}/100",format!("{:}",percent_of_consume));    
      }
      else{
        print!("{}/100",format!("{:.2}",percent_of_consume));
      }
      
      self.stdout.flush().unwrap();
    }
}
struct Progress{
  total_size:usize,
  consumed_size:usize,
  approximate_progres:usize,
  progress_bar:ProgressBar,
  number_of_bars:usize
}
impl Progress {
  fn from_total_size(total_size:usize)->Self{
      const NUMBER_OF_BARS:usize=25;
      Self{
        total_size,
        consumed_size:0,
        approximate_progres:0,
        progress_bar:ProgressBar::progress_bar(NUMBER_OF_BARS),
        number_of_bars:NUMBER_OF_BARS
      }
  }
  fn percent_of_consume(&self)->f64{
    (self.consumed_size as f64)/(self.total_size as f64)
  }
  fn percent_of_progress(&self)->f64{
    (self.approximate_progres as f64)/(self.number_of_bars as f64)
  }
  fn consume(&mut self,lenght:usize){
    self.consumed_size+=lenght;
    if self.percent_of_consume()>self.percent_of_progress(){
        self.approximate_progres+=1;
        self.progress_bar.draw_a_bar(self.approximate_progres);
    }
    self.progress_bar.change_progress_number(self.percent_of_consume())
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

