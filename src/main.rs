use std::io;

mod new_pass;
mod search_name;
mod index;
mod text_template;

fn main() {
    text_template::home::home_presentation();

    loop {
        text_template::home::home_options();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

            match input.trim().parse::<i32>() {
                Ok(num) => {
                    match num {
                        1 => new_pass::new_pass(),
                        2 => index::index(),
                        3 => search_name::search_name(),
                        9 => {
                            println!("Exiting the program.");
                            return; // Exit the program if input is 9
                        }
                        _ => {
                            println!("Input is not one of the options! Please try again.");
                            continue; // Restart the loop for any other input
                        }
                    }
                    break;
                }
                Err(_) => {
                    println!("Input is not a number! Please try again.");
                    continue; // Restart the loop if input is not a number
                }
            }
        }
}
