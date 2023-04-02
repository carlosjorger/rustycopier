
use std::{env, time::Instant};

mod buffer_reader{
  use std::fs::File;
  use std::io::{BufReader, BufRead};
 pub struct BufferReader{
      curr: u128,
      next: u128,
      reader: BufReader<File>,
      capacity:usize
  }
  
  impl Iterator for BufferReader{

      type Item= u128;
      fn next(&mut self) -> Option<Self::Item> {
        
        let buffer=self.reader.fill_buf().expect("error in the buffer");
        let lenght= buffer.len();
        if lenght==self.capacity {
            self.curr=self.next;
            self.next=u128::from_be_bytes(buffer.try_into().expect("incorrect length"));
            self.reader.consume(lenght);
            Some(self.curr)
        }
        else if lenght<self.capacity&& lenght>0 {
            self.curr=self.next;
            let mut complete_buffer:Vec<u8>=vec![0;self.capacity-lenght];
            let mut incomplete_buffer= buffer.to_vec();
            complete_buffer.append(&mut incomplete_buffer);
            self.next=u128::from_be_bytes(complete_buffer.try_into().expect("incorrect length"));
            self.reader.consume(lenght);
            Some(self.curr)
        }
        else{
           None
        }
        
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
    let source_path=&args[1];
    let buffer_reader=get_buffer(source_path);
    let start = Instant::now();
    for buffer in buffer_reader {
        //  println!("> {:?}", buffer);
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

