use crate::text_template;
use std::io;

pub fn new_pass() {

  loop {
    text_template::new_pass::name_tag();
    let tag:String = get_tag();
  
    text_template::new_pass::name_user();
    let user:String = get_username();
  
    text_template::new_pass::name_password();
    let pass:String = get_pass();
  
    text_template::new_pass::description();
    let description:String = get_description();
  
    text_template::new_pass::show_result(&tag, &user, &pass, &description);

    let confirmation: String = get_confirmation();

  }
}

fn get_tag()->String {
  let mut tag = String::new();

  io::stdin()
    .read_line(&mut tag)
    .expect("Failed to read imput");

  tag
}

fn get_username()->String {
  let mut user_login: String = String::new();

  io::stdin()
    .read_line(&mut user_login)
    .expect("Failed to read imput");

  user_login
}

fn get_pass()->String {
  let mut user_pass:String = String::new();

  io::stdin()
    .read_line(&mut user_pass)
    .expect("Failed to read imput");

  user_pass
}

fn get_description()-> String {
  let mut description:String = String::new();

  io::stdin()
    .read_line(&mut description)
    .expect("Failed to read imput");

  description
}

fn get_confirmation()-> String {
  let mut confirmation = String::new();
  
  loop {
    text_template::new_pass::confirmation();

    io::stdin()
    .read_line(&mut confirmation)
    .expect("Failed to read imput");

    match confirmation.as_str() {
      "y"| "Y" => {save_data();}
      "n" | "N" => {edit_data();}
      "q" | "Q" => {println!("Exiting the program.");}
      _ => println!("Invalid imput")
    }
  }
}

fn save_data() {
  println!("saved");

  "Saved";
}

fn edit_data() {
  println!("editting");
  "y";
}