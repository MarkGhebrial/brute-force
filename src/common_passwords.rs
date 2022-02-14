/// Get the passwords in the common password list that are of the length
/// stored in the given PasswordParameters struct
pub fn get_common_passwords_of_len(len: usize) -> Vec<String> {
    include!("../password-list.txt") // Macro to read the contents of a file at compile-time and dump the expression here
        .split("\n") // Get each line
        .filter_map(|s| { // "Filter" through the list...
            let mut out: Option<String> = None;
            if s.len() == len { // ... excluding any items with the wrong length
                out = Some(s.to_string());
            }
            out
        })
        .collect() // Return a vector
}