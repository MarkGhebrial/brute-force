mod password_parameters;
mod password_generator;
mod user_input;

use password_parameters::*;
use password_generator::*;
use user_input::*;

fn main() {
    let params = PasswordParameters::prompt_user();

    let num_of_combos = params.num_of_possible_combos();

    if prompt_for_boolean(&format!("There are {} possible combinations. Continue?", num_of_combos)) {
        brute_force(&params);
    } else {
        println!("OK");
    }  
}

fn brute_force (params: &PasswordParameters) {
    let password: Password = Password::new(&params);
    let num_of_combos = params.num_of_possible_combos();

    let mut attempt_no = 0;
    for (i, combo) in params.combinations().enumerate() {
        println!("Trying {}; Elapsed time: _ ; Attempt no. {} of {}", combo, i, num_of_combos);
        if combo == password.password {
            attempt_no = i;
            break;
        }
    }

    println!("Cracked password '{}' on attempt {} in _ seconds", password.password, attempt_no);
}