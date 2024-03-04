use crate::text_template;
use std::io;

pub fn new_pass() {
  text_template::new_pass::name_pass();

  let mut pass_name = String::new();

  io::stdin()
    .read_line(&mut pass_name)
    .expect("Failed to read imput");
}

// fn name_pass()-> String{

// }