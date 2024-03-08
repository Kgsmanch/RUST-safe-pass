use std::io;

pub fn search_name() {
  let mut name:String = String::new(); 

  io::stdin()
    .read_line(&mut name)
    .expect("failed to read input");

  
}