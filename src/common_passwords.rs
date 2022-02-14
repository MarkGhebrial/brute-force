/// Get the passwords in the common password list that are of the length
/// stored in the given PasswordParameters struct
pub fn get_common_passwords_of_len(len: usize) -> Vec<String> {
    include!("../password-list.txt")
        .split("\n")
        .filter_map(|s| {
            let mut out: Option<String> = None;
            if s.len() == len {
                out = Some(s.to_string());
            }
            out
        })
        .collect()
}