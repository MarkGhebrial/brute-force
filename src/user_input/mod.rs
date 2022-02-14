mod general_prompts;
pub use general_prompts::*;

mod password_prompts;
pub use password_prompts::*;

/// Keep running the given closure until it returns `Result::Ok<T>`.
/// 
/// Every time `action` returns an `Err`, the closure
/// `handle_err` will be run with that error passed as a prameter.
pub fn retry_until_ok<F, T, E, H>(action: F, handle_error: H) -> T 
    where 
        F: Fn() -> Result<T, E>, 
        H: Fn(E)
{
    let mut out: Option<T> = None;
    while let None = out {
        match action() {
            Ok(v) => out = Some(v),
            Err(e) => handle_error(e)
        }
    }
    out.unwrap()
}