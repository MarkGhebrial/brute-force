use std::io;

// TODO: use `retry_until_ok` in more of these functions

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

/// Keep running the given closure until it returns `Result::Ok<T>`.
/// 
/// Every time `action` returns an `Err`, the closure
/// `handle_err` will be run with that error passed as a prameter.
pub fn retry_until_ok<F, T, E, H>(action: F, handle_error: H) -> T 
    where 
        F: Fn() -> Result<T, E>, 
        H: Fn(E)
{
    let mut out: Option<T> = None;
    while let None = out {
        match action() {
            Ok(v) => out = Some(v),
            Err(e) => handle_error(e)
        }
    }
    out.unwrap()
}