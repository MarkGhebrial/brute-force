use std::io;

use super::retry_until_ok;

/// Ask the user for a positive interger, retrying
/// if they provide invalid input
pub fn prompt_for_uint (prompt: &str) -> usize {
    if prompt.len() > 0 {
        println!("{} ", prompt);
    }

    retry_until_ok(
        || {
            let input = prompt_for_string(""); // Ask for a string
            input.parse::<usize>() // Parse that string into a unsigned integer
        }, 
        |_| println!("Please enter a positive integer"))
}

/// Ask the user a yes or no question, returning their
/// response as a boolean
pub fn prompt_for_boolean (prompt: &str) -> bool {
    println!("{} (yes/no) ", prompt);

    retry_until_ok(
        || {
            let input = prompt_for_string("");
            match input.to_lowercase().as_str() {
                "yes" | "y" => Ok(true),
                "no" | "n" => Ok(false),
                _ => Err(())
            }
        },
        |_| println!("Please enter \"yes\" or \"no\"")
    )
}

/// Present the user with a number of options specified 
/// in a vector of string references
pub fn menu_prompt(prompt: &str, options: Vec<&str>) -> usize {
    println!("{} (0-{})", prompt, options.len() - 1);
    for (i, s) in options.iter().enumerate() {
        println!("  {}) {}", i , s);
    }

    let mut out: Option<usize> = None;
    while let None = out {
        let input = prompt_for_uint("");
        if input < options.len() {
            out = Some(input);
        } else {
            println!("Please enter a number between 0 and {}", options.len() - 1);
        }
    }

    out.unwrap()
}

/// Ask the user to input a string
pub fn prompt_for_string(prompt: &str) -> String {
    if prompt.len() > 0 {
        println!("{}", prompt);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Read the user's input
    let input = input.trim(); // Remove whitespace

    input.to_string()
}