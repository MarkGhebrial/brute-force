mod password_parameters;
mod password_generator;
mod user_input;
mod brute_force;

use password_parameters::*;
use password_generator::*;
use user_input::*;
use brute_force::*;

fn main() {
    let mut password: Option<Password> = None;

    let generation_strategy = menu_prompt("How would you like to generate a password?", vec![
        "Randomly generated",
        "User specified",
        "Memorable password"
    ]);
    match generation_strategy {
        0 => password = Some(prompt_random_password()),
        1 => password = Some(prompt_user_specified_password()),
        2 => println!("Coming soon!"),
        _ => {}
    };

    let password = password.unwrap();

    let num_of_combos = &password.parameters.num_of_possible_combos();

    if prompt_for_boolean(&format!("There are {} possible combinations. Continue?", num_of_combos)) {
        brute_force(&password);
    } else {
        println!("OK");
    }  
}

/// Ask the user to specify the parameters for a random 
/// password, then generate that password
fn prompt_random_password() -> Password {
    println!("Please specify the parameters for the random password");
    let params = PasswordParameters::prompt_user();
    Password::random(params)
}

/// Ask the user to input their own password, retrying if
/// it contains an invalid character
fn prompt_user_specified_password() -> Password {
    let mut password: Option<Password> = None;

    while let None = password {
        let user_password = Password::from_str(&prompt_for_string("Please enter a password:"));
        match user_password {
            Ok(p) => password = Some(p),
            Err(e) => println!("Invalid character '{}'", e.0)
        };
    }
    password.unwrap()
}