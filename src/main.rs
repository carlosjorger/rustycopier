use std::fs;
use std::env;
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
    let file_path=&args[1];
        let contents = fs::read(file_path)
          .expect("Should have been able to read the file");

    for data in contents {
      println!("{data}");
    }
}
