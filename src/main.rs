mod password_parameters;
mod password_generator;
mod user_input;
mod brute_force;
mod common_passwords;

use password_parameters::*;
use password_generator::*;
use user_input::*;
use brute_force::*;
use common_passwords::*;

// The password generation project, but as a library
use password_gen_lib::prompt_for_memorable_password;

fn main() {
    // Ask the user how to generate the password
    let generation_strategy = menu_prompt("How would you like to generate a password?", vec![
        "Randomly generated",
        "User specified",
        "Memorable password"
    ]);

    // Generate the password based on the user-specified strategy
    let password = match generation_strategy {
        0 => Some(prompt_random_password()),
        1 => Some(prompt_user_specified_password()),
        2 => Some(prompt_memorable_password()),
        _ => None // This case should never happen...
    };
    let password = password.unwrap(); // ...so we can panic if it does

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
    retry_until_ok(|| {
        Password::from_str(&prompt_for_string("Please enter a password:"))
    }, |error| {
        println!("Invalid character '{}'", error.0);
    })
}

/// Ask the user to generate a memorable password
fn prompt_memorable_password() -> Password {
    retry_until_ok(|| {
        Password::from_str(&prompt_for_memorable_password())
    }, |error| {
        println!("Invalid character '{}'", error.0);
    })
}