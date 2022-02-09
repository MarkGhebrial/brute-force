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
        0 => {
            println!("Please specify the parameters for the random password");
            let params = PasswordParameters::prompt_user();
            password = Some(Password::random(params));
        }
        1 => {
            while let None = password {
                let user_password = Password::from_str(&prompt_for_string("Please enter a password"));
                match user_password {
                    Ok(p) => password = Some(p),
                    Err(_) => println!("Please try again")
                };
            }
        }
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