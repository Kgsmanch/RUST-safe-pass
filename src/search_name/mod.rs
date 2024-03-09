use std::fs::read_to_string;
use std::io;
use crate::text_template;

pub fn search_name() {

  
  text_template::search_text::search_name_header();
  let mut name:String = String::new(); 

  io::stdin()
    .read_line(&mut name)
    .expect("failed to read input");

  name = name.trim().to_string();

  let file = read_to_string(".data.txt").unwrap();

  for line in file.lines() {
    if line.contains(&name) {
      println!("{}", line);
    }
  }

  println!("\n")
}