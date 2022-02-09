use rand::prelude::*;
use crate::password_parameters::{
    PasswordParameters,
    InvalidCharacter,
};

/// Holds a string and the PasswordParameters struct that
/// describes it
pub struct Password {
    pub password: String,
    pub parameters: PasswordParameters
}

impl Password {
    /// Return a new, randomly generated password with the parameters
    /// supplied
    pub fn random(parameters: PasswordParameters) -> Password {
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

    pub fn from_str(s: &str) -> Result<Password, InvalidCharacter> {
        Ok(Password {
            password: String::from(s),
            parameters: PasswordParameters::from_str(s)?,
        })
    }
}