use std::collections::HashMap;
use super::PasswordParameters;

/// An interator that yields every possible password with
/// the specified parameters
/// 
/// This iterator is looping, so when the last combination
/// is reached, it will roll back to the first one. (i.e. 
/// zzzy -> zzzz -> aaaa -> aaab)
pub struct Combinations {
    current_combo: String,

    alphabet: HashMap<usize, char>,
    tebahpla: HashMap<char, usize>,
}

impl Combinations {
    /// Return a new iterator over all possible password combinations
    /// with the specified parameters
    pub fn new (params: &PasswordParameters) -> Combinations {
        // Start with the first possible character repeated for
        // the length of the password
        let mut starting_combo = String::new();
        for _ in 0..params.length {
            starting_combo.push(params.get_list_of_possible_characters()[0]);
        }

        // Map the vector of possible characters into two HashMaps
        // for faster lookup
        let mut alphabet = HashMap::new();
        let mut tebahpla = HashMap::new();
        let list_of_chars = params.get_list_of_possible_characters();
        for (i, c) in list_of_chars.iter().enumerate() {
            alphabet.insert(i, *c);
            tebahpla.insert(*c, i);
        }

        Combinations {
            current_combo: starting_combo,
            alphabet,
            tebahpla,
        }
    }

    /// "Increment" current_combo by one
    /// 
    /// This can be though as adding one to a base n number, where
    /// n is the amount of possible characters that can be used in
    /// the password and each character is a digit. 
    fn increment (&mut self) {
        let mut out = String::new();

        let mut rollover = true; // If the current character "rolls over" to the first one, then the one preceeding it must also be incremented
        for c in self.current_combo.chars().rev() { // Start with the rightmost character
            if rollover {
                let mut num = *self.tebahpla.get(&c).unwrap(); // Get the index of the current char
                num += 1; // Increment that index (i.e. move the the next char in the list)

                match self.alphabet.get(&num) { // Get the character for index num
                    Some(c) => { // If the index is valid, then the character is OK and will simply be replaced
                        out.push(*c); // Append the new character
                        rollover = false; // No rollover (z -> a) occured, so the next character does not need to be incremented
                    },
                    None => { // If the index is invalid, then is is too high and must be "rolled over" to zero
                        out.push(*self.alphabet.get(&0).unwrap()); // Append the character at index zero
                        rollover = true; // Rollover of the current character occured, so the next character must be incremented
                    }
                }
            } else {
                out.push(c);
            }
        }

        out = out.chars().rev().collect();
        self.current_combo = out;
    }
}

impl Iterator for Combinations {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = String::from(&self.current_combo);

        self.increment();

        Some(out)
    }
}