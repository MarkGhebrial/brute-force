use super::Combinations;

pub fn get_lowercase_letters() -> Vec<char> { "abcdefghijklmnopqrstuvwxyz".chars().collect() }
pub fn get_uppercase_letters() -> Vec<char> { "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect() }
pub fn get_digits() -> Vec<char> { "0123456789".chars().collect() }
pub fn get_special_characters() -> Vec<char> { "~`!@#$%^&*()_-+=[]{}\\|;:<>,./?".chars().collect() }

/// Represents the parameters for a password
pub struct PasswordParameters {
    pub length: usize,
    pub lowercase_letters: bool,
    pub uppercase_letters: bool,
    pub digits: bool,
    pub special_characters: bool,
}

impl PasswordParameters {
    /// Ask the user for the parameters of a password and return their response
    pub fn prompt_user() -> PasswordParameters {
        use crate::user_input::{
            prompt_for_boolean,
            prompt_for_uint
        };

        let lowercase_letters = prompt_for_boolean("Use lowercase letters?");
        let uppercase_letters = prompt_for_boolean("Use uppercase letters?");
        let digits = prompt_for_boolean("Use numerals?");
        let special_characters = prompt_for_boolean("Use special characters?");

        let length = prompt_for_uint("How long do you want the password to be? (4, 6, or 8)");

        PasswordParameters { 
            length, 
            lowercase_letters, 
            uppercase_letters, 
            digits,
            special_characters,
        }
    }

    /// Detects the parameters from a string
    pub fn from_str(s: &str) -> Result<PasswordParameters, InvalidCharacter> {
        let length = s.len();
        let mut lowercase_letters = false;
        let mut uppercase_letters = false;
        let mut digits = false;
        let mut special_characters = false;

        // For each character in the string, check which character set
        // it belongs to
        for c in s.chars() {
            if get_lowercase_letters().contains(&c) {
                lowercase_letters = true
            } else if get_uppercase_letters().contains(&c) {
                uppercase_letters = true;
            } else if get_digits().contains(&c) {
                digits = true;
            } else if get_special_characters().contains(&c) {
                special_characters = true;
            } else {
                return Err(InvalidCharacter(c));
            }
        }

        Ok(PasswordParameters {
            length, 
            lowercase_letters, 
            uppercase_letters, 
            digits,
            special_characters,
        })
    }

    /// Calculate the number of possible passwords for a set
    /// of parameters
    pub fn num_of_possible_combos(&self) -> usize {
        let num_of_possible_chars = self.get_list_of_possible_characters().len();

        num_of_possible_chars.pow(self.length as u32)
    }

    /// Return a vector that contains all the valid characters
    /// for the parameters (self)
    pub fn get_list_of_possible_characters(&self) -> Vec<char> {

        let mut out = vec![];

        if self.lowercase_letters {
            out.append(&mut get_lowercase_letters());
        }
        if self.uppercase_letters {
            out.append(&mut get_uppercase_letters());
        }
        if self.digits {
            out.append(&mut get_digits());
        }
        if self.special_characters {
            out.append(&mut get_special_characters());
        }

        out
    }

    /// Return an iterator over all possible password combinations
    /// with the current parameters
    pub fn combinations(&self) -> Combinations {
        Combinations::new(&self)
    }
}

/// Returned when the user inputs a password with an unsupported character (like a space)
pub struct InvalidCharacter(pub char);