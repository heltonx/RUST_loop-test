//Simple Rust program that prompts the user for their name and then greets them.
use std::io;



fn main() {

    // Prompt the user for their name

    println!("Hello! What's your name?");



    // Create a mutable string to store the user's name

    let mut name = String::new();



    // Read the user's input and handle potential errors

    match io::stdin().read_line(&mut name) {

        Ok(_) => {

            // Trim any leading or trailing whitespace from the input

            let name = name.trim();



            // Check if the user entered a name

            if !name.is_empty() {

                // Greet the user

                println!("Hello, {}! Nice to meet you.", name);

            } else {

                // Handle the case where the user did not enter a name

                println!("Oops! It seems you didn't enter a valid name.");

            }

        }

        Err(e) => {

            // Handle any errors that occurred during input

            println!("Error reading input: {}", e);

        }

    }

}