use std::fs::File;
use std::io::{self, BufRead};

pub fn delete() {
  read_file_and_print(".data.txt")
  .unwrap_or_else(|err| eprintln!("Error: {}", err));

  exclude_file_and_print(".data.txt");
}

fn read_file_and_print(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    println!("\n\n");

    for (line_number, line) in reader.lines().enumerate() {
      let line_number = line_number + 1;
      let line = line?;
      println!("[{}]= {}", line_number, line);
    }
    println!("\n\n");

    Ok(())
}

fn exclude_file_and_print(filename: &str) -> io::Result<()>{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    println!("\n\n");

    
    Ok(())
}