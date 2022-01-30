mod password_parameters;
mod password_generator;
mod user_input;

use password_parameters::*;
use password_generator::*;
use user_input::*;

use std::time::Instant;

fn main() {
    // Ask the user for the password's parameters
    let params = PasswordParameters::prompt_user();

    let num_of_combos = params.num_of_possible_combos();

    if prompt_for_boolean(&format!("There are {} possible combinations. Continue?", num_of_combos)) {
        brute_force(&params);
    } else {
        println!("OK");
    }  
}

/// Generate a random password and try to brute-force it, printing
/// every attempt.
fn brute_force (params: &PasswordParameters) {
    let password: Password = Password::new(&params); // Generate a random password
    let num_of_combos = params.num_of_possible_combos();

    let starting_time = Instant::now(); // Start the timer

    // Values will be written to these variables once the password has been guessed correctly
    let mut attempt_no = 0; // Which number attempt we found the password on
    let mut attempt_time = 0.0; // How long it took

    for (i, combo) in params.combinations().enumerate() { // Loop through every possible combination
        let elapsed_time = starting_time.elapsed().as_secs_f64();

        // Print some useful information (this it the slowest part of the program)
        println!(
            "Trying {} ; Elapsed time: {:.3} seconds ; Attempt no. {} of {} ({:.1} attempts per second)", 
            combo, 
            elapsed_time, 
            i, 
            num_of_combos, 
            i as f64 / elapsed_time
        );

        // Check if the guessed password matches the randomly generated one
        if combo == password.password {
            attempt_no = i;
            attempt_time = elapsed_time;
            break; // Leave the loop
        }
    }

    println!("Cracked password '{}' on attempt {} in {:.5} seconds", password.password, attempt_no, attempt_time);
}