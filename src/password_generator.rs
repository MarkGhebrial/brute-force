use rand::prelude::*;
use crate::password_parameters::PasswordParameters;

/// Holds a string and the PasswordParameters struct that
/// describes it
pub struct Password<'a> {
    pub password: String,
    pub parameters: &'a PasswordParameters
}

impl Password<'_> {
    /// Return a new, randomly generated password with the parameters
    /// supplied
    pub fn new (parameters: &PasswordParameters) -> Password {
        let possible_characters = parameters.get_list_of_possible_characters();

        let mut password = String::new();
        for _ in 0..parameters.length {
            let rand_char = possible_characters[
                rand::thread_rng().gen_range(0..possible_characters.len()) // Randomly choose a character
            ];

            password.push(rand_char); // Append the random charater to the password
        }

        Password {
            password,
            parameters,
        }
    }
}