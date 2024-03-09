use ansi_term::Colour::Red;

pub fn search_name_header() {
  println!("
  -> Please, insert a name for search
  {}
  ", Red.paint("Insert Search Name"))
}
