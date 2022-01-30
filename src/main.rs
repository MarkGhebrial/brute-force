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

        let password: Password = Password::new(&params);
        println!("{}", password.password);

        for combo in params.combinations() {
            println!("{}", combo);
            if combo == password.password {
                break;
            }
        }

        println!("noice");

    } else {
        println!("OK");
    }  
}