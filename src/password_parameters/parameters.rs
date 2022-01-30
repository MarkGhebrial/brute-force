pub struct PasswordParameters {
    pub length: usize,
    pub lowercase_letters: bool,
    pub uppercase_letters: bool,
    pub digits: bool,
    pub special_characters: bool,
}

impl PasswordParameters {
    pub fn prompt_user () -> PasswordParameters {
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

    /// Calculate the number of possible passwords for a set
    /// of parameters
    pub fn num_of_possible_combos (&self) -> usize {
        let num_of_possible_chars = self.get_list_of_possible_characters().len();

        num_of_possible_chars.pow(self.length as u32)
    }

    pub fn get_list_of_possible_characters (&self) -> Vec<char> {

        let mut out = vec![];

        if self.lowercase_letters {
            out.append(&mut "abcdefghijklmnopqrstuvwxyz".chars().collect());
        }
        if self.uppercase_letters {
            out.append(&mut "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect());
        }
        if self.digits {
            out.append(&mut "0123456789".chars().collect());
        }
        if self.special_characters {
            out.append(&mut "~`!@#$%^&*()_-+=[]{}\\|;:<>,./?".chars().collect());
        }

        out
    }

    pub fn combinations (&self) -> Combinations {
        Combinations::new(&self)
    }
}