use std::collections::HashMap;
use super::PasswordParameters;

/// An interator that yields every possible password with
/// the specified parameters
pub struct Combinations {
    current_combo: String,

    alphabet: HashMap<usize, char>,
    tebahpla: HashMap<char, usize>,
}

impl Combinations {
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

    fn increment (&mut self) {
        let mut out = String::new();

        let mut rollover = true; // If the current character "rolls over" to the first one, then the one preceeding it must also be incremented
        for c in self.current_combo.chars().rev() {
            if rollover {
                let mut num = *self.tebahpla.get(&c).unwrap();
                num += 1;

                match self.alphabet.get(&num) {
                    Some(c) => {
                        out.push(*c);
                        rollover = false;
                    },
                    None => {
                        out.push(*self.alphabet.get(&0).unwrap());
                        rollover = true; // Overflow into the next character
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