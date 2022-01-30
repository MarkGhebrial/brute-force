use rand::prelude::*;
use crate::password_parameters::PasswordParameters;

pub struct Password<'a> {
    pub password: String,
    pub parameters: &'a PasswordParameters
}

impl Password<'_> {
    pub fn new (parameters: &PasswordParameters) -> Password {
        let possible_characters = parameters.get_list_of_possible_characters();

        let mut password = String::new();
        for _ in 0..parameters.length {
            let rand_char = possible_characters[
                rand::thread_rng().gen_range(0..possible_characters.len())
            ];

            password.push(rand_char);
        }

        Password {
            password,
            parameters,
        }
    }

    /*pub fn brute_force () -> f64 {
        //for 

        0.0
    }*/
}