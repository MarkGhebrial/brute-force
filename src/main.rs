mod password_parameters;
mod password_generator;
mod user_input;

use password_parameters::*;
use password_generator::*;

fn main() {
    let params = PasswordParameters::prompt_user();

    let num_of_combos = params.num_of_possible_combos();
    println!("That's a lot! {}", num_of_combos);

    let password: Password = Password::new(params);
    println!("{}", password.password);

    for i in 0..num_of_combos {
        println!("{:?} / {:?}", i, num_of_combos);
    }
}