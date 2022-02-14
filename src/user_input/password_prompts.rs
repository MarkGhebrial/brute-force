use crate::*;

// The password generation project, but as a library
use password_gen_lib::prompt_for_memorable_password;

/// Ask the user to specify the parameters for a random 
/// password, then generate that password
pub fn prompt_random_password() -> Password {
    println!("Please specify the parameters for the random password");
    let params = PasswordParameters::prompt_user();

    let out = Password::random(params);
    println!("Generated password: {}", out.password);
    out
}

/// Ask the user to input their own password, retrying if
/// it contains an invalid character
pub fn prompt_user_specified_password() -> Password {
    retry_until_ok(|| {
        Password::from_str(&prompt_for_string("Please enter a password:"))
    }, |error| {
        println!("Invalid character '{}'", error.0);
    })
}

/// Ask the user to generate a memorable password
pub fn prompt_memorable_password() -> Password {
    let out = retry_until_ok(|| {
        Password::from_str(&prompt_for_memorable_password())
    }, |error| {
        println!("Invalid character '{}'", error.0);
    });

    println!("Generated password: {}", out.password);
    out
}