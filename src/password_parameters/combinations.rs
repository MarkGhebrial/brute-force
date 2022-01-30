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

    fn increment
}

impl Iterator for Combinations<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = String::from(&self.current_combo);

        Some(out)
    }
}