use std::fs::File;
use std::io::{self, BufRead};

pub fn index() {
  read_file_and_print(".data.txt")
  .unwrap_or_else(|err| eprintln!("Error: {}", err));
}

fn read_file_and_print(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    println!("\n\n");
    for line in reader.lines() {
        println!("{}", line?);
    }
    println!("\n\n");

    Ok(())
}
