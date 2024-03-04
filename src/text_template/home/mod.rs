use ansi_term::Colour::{Red, Green};

pub fn home_presentation() {
  println!("\n\n-> {}\n-> Choose your option by the number", 
  Green.underline().paint("Welcome to Safe Pass"));
}

pub fn home_options() {
  println!("[{}]Insert New Pass | [{}]Search Through Index | [{}]Search By Name | [{}]Quit Program", 
  Red.paint("1"), 
  Red.paint("2"), 
  Red.paint("3"), 
  Red.paint("9"));
}
