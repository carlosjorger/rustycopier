
use std::fs::File;
use std::path::Path;
use std::{env, time::Instant};
use std::io::{BufWriter, Write, BufReader, BufRead};
mod buffer_reader{
  use std::fs::File;
  use std::io::{BufReader, BufRead};
//  pub struct BufferReader<'a>{
 pub struct BufferReader{

      curr: u128,
      next: u128,
      reader: BufReader<File>,
      capacity:usize,
      // e: &'a [u8]
  }
  
  // impl<'a> Iterator for BufferReader<'a>{
  impl Iterator for BufferReader{


      type Item= u128;
      fn next(&mut self) -> Option<Self::Item> {
        
        let buffer=self.reader.fill_buf().expect("error in the buffer");
        let lenght= buffer.len();
        if lenght==0 {
            return None;
        }
        self.curr=self.next;
        if lenght==self.capacity {
            self.next=u128::from_be_bytes(buffer.try_into().expect("incorrect length"));
        }
        else {
            let mut complete_buffer:Vec<u8>=vec![0;self.capacity-lenght];
            let mut incomplete_buffer= buffer.to_vec();
            complete_buffer.append(&mut incomplete_buffer);
            self.next=u128::from_be_bytes(complete_buffer.try_into().expect("incorrect length"));
          }
        self.reader.consume(lenght);
        Some(self.curr)
      }
  }
  pub fn get_buffer(source_path:&str)->BufferReader{
    let file = File::open(source_path)
          .expect("Should have been able to read the file");
    let reader=BufReader::with_capacity(16, file);
    BufferReader { curr: 0, next: 0, reader: reader, capacity: 16 }
  }
}

use crate::buffer_reader::get_buffer;
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
    let mut stream =BufWriter::with_capacity(16, file);
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

