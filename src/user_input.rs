use std::io;

/// Ask the user for a positive interger, retrying
/// if they provide invalid input
pub fn prompt_for_uint (prompt: &str) -> usize {
    let mut pass_len: Option<usize> = None;
    println!("{} ", prompt);
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