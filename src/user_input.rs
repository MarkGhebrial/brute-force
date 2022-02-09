use std::io;

/// Ask the user for a positive interger, retrying
/// if they provide invalid input
pub fn prompt_for_uint (prompt: &str) -> usize {
    let mut pass_len: Option<usize> = None;

    if prompt.len() > 0 {
        println!("{} ", prompt);
    }

    while let None = pass_len {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read the user's input
        let input = input.trim(); // Remove whitespace
        
        match input.parse::<usize>() { // Parse the String into an integer
            Ok(n) => pass_len = Some(n),
            Err(_) => println!("Please enter a positive integer") // If the string cannot be parsed
        };
    }

    pass_len.unwrap()
}

/// Ask the user a yes or no question, returning their
/// response as a boolean
pub fn prompt_for_boolean (prompt: &str) -> bool {
    let mut out: Option<bool> = None;
    println!("{} (yes/no) ", prompt);
    while let None = out {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read the user's input
        let input = input.trim();

        match input.to_lowercase().as_str() {
            "yes" | "y" => out = Some(true),
            "no" | "n" => out = Some(false),
            _ => println!("Please enter \"yes\" or \"no\"")
        };
    }

    out.unwrap()
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

pub fn prompt_for_string(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Read the user's input
    let input = input.trim(); // Remove whitespace

    input.to_string()
}