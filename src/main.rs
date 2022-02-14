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