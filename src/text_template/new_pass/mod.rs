use ansi_term::Colour::{Red, Yellow, Green};

pub fn name_tag() {
  println!("
  -> Please, give a Tag to your registry.
  -> Obs: its only permmited single names for example 'github'.
  -> for composed name you must use underscore for example 'ssh_my_project'.

  {}
  ", Red.paint("Insert Registry Name"))
}

pub fn name_user() {
  println!("
  -> Please, give the username.
  -> Obs: its only permmited single names for example: 
        'MyGithub', 
      email: 'example@mail.com', 
      or even sequence of numbers without empty spaces: 0987654378.

  {}
  ", Red.paint("Insert Username"))
}

pub fn name_password() {
  println!("
  -> Please, give the password.
  {}", Red.paint("Insert Password"))
}

pub fn description() {
  println!("
  -> It is {}.
  {}", Green.underline().paint("optional"), Yellow.paint("Insert Description"))
}

pub fn show_result(tag:&String, user:&String, pass:&String, description:&String) {
  println!("
  ____________________________________
  -> Registry Name: {}
  -> Username: {}
  -> Password: {}
  -> Description: {}", 
  tag, user, pass, description)
}

pub fn confirmation() {
  println!("{}",  Yellow.paint(
    "Please:
      Type y/Y to Save
      Type n/N to Edit
      Type q/Q to Exit Program"))
}


