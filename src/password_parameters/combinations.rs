use std::collections::HashMap;

/// An interator that yields every possible password with
/// the specified parameters
struct Combinations<'a> {
    params: &'a PasswordParameters,
    current_combo: String
}

impl Combinations<'_> {
    pub fn new (params: &PasswordParameters) -> Combinations {
        // Start with the first possible character repeated for
        // the length of the password
        let mut starting_combo = String::new();
        for _ in 0..params.length {
            starting_combo.push(params.get_list_of_possible_characters()[0]);
        }

        Combinations {
            params,
            current_combo: starting_combo
        }
    }

    fn increment (&self) {
        let mut alphabet = HashMap::new();
        let mut tebahpla = HashMap::new();
        for (i, c) in params.get_list_of_possible_characters().enumerate() {
            alphabet.insert(i, c);
            tebahpla.insert(c, i);
        }

        let mut out = String::new();

        let mut rollover = true;
        for c in current_combo.chars().rev() {
            if rollover {
                let mut num = tebahpla.get(&c).unwrap();
                num += 1;

                match alphabet.get(&num) {
                    Some(c) => {
                        out.push(c);
                        rollover = false;
                    },
                    None => {
                        out.push(alphabet.get(0).unwrap());
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

impl Iterator for Combinations<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = String::from(&self.current_combo);

        self.increment();

        Some(out)
    }
}