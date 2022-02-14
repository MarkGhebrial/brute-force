use crate::Password;
use crate::get_common_passwords_of_len;

use std::time::Instant;
use std::sync::mpsc;
use std::thread;

/// Generate a random password and try to brute-force it, printing
/// every attempt.
pub fn brute_force (password: &Password) {
    let params = &password.parameters; // Generate a random password
    let num_of_combos = params.num_of_possible_combos();

    let starting_time = Instant::now(); // Start the timer

    // Values will be written to these variables once the password has been guessed correctly
    let mut attempt_no = 0; // Which number attempt we found the password on
    let mut attempt_time = 0.0; // How long it took

    // Create an iterator to loop over the common password list and all possible passwords
    let password_iterator = get_common_passwords_of_len(params.length)
        .into_iter()
        .chain(params.combinations());

    for (i, combo) in password_iterator.enumerate() { // Loop through every possible combination
        let elapsed_time = starting_time.elapsed().as_secs_f64();

        // Print some useful information (this it the slowest part of the program)
        println!(
            "Trying {} ; Elapsed time: {:.3} seconds ; Attempt no. {} of {} ({:.1} attempts per second)", 
            combo, 
            elapsed_time, 
            i, 
            num_of_combos, 
            i as f64 / elapsed_time
        );

        // Check if the guessed password matches the randomly generated one
        if combo == password.password {
            attempt_no = i;
            attempt_time = elapsed_time;
            break; // Leave the loop
        }
    }

    println!("Cracked password '{}' on attempt {} in {:.5} seconds", password.password, attempt_no, attempt_time);
}

/// Spawn a new thread to brute force a password, returning
/// a Receiver<Stirng>.
///
/// The String in the received tuple is the guesed password
fn spawn_brute_force_thread(correct_password: &Password, combo_sender: &mpsc::Sender<String>) -> mpsc::Sender<bool> {
    let combos = correct_password.parameters.combinations();

    let cloned_sender = combo_sender.clone();

    // Mpsc channel to kill the thread. If the sender sends true, then the thread will stop
    let (kill_sender, kill_reciever) = mpsc::channel();

    thread::spawn(move || {
        for combo in combos {
            cloned_sender.send(combo).unwrap();

            match kill_reciever.recv() {
                Ok(b) => if b { break; },
                Err(_) => break,
            };
        }
    });

    kill_sender
}