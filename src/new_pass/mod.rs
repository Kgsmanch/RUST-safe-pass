use crate::text_template;
use std::io;
use std::process;

mod registry;

pub fn new_pass() {

  'root:loop {
    text_template::new_pass::name_tag();
    let tag:String = get_tag();
  
    text_template::new_pass::name_user();
    let user:String = get_username();
  
    text_template::new_pass::name_password();
    let pass:String = get_pass();
  
    text_template::new_pass::description();
    let description:String = get_description();
  
    text_template::new_pass::show_result(&tag, &user, &pass, &description);

    let result = get_confirmation();
    if result == true {
      let _save = registry::registry(&tag, &user, &pass, &description);
      break 'root
    };
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

fn get_confirmation()->bool {
  let mut confirmation = String::new();
  
  loop {
    text_template::new_pass::confirmation();

    io::stdin()
    .read_line(&mut confirmation)
    .expect("Failed to read imput");

    let option = confirmation.trim().to_lowercase();

    if option == "y" {
      return true;
    }

    if option == "n" {
      break false;
    }

    if option == "q" {
      println!("Exiting program...");
      process::exit(0);
    }
  }
}
