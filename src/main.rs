extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number game! Input your guess:");

    let secret_number = rand::thread_rng().gen_range(0, 100);
    let mut attempts: u32 = 0;

    loop {
        attempts += 1;
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result_text;

        match guess.cmp(&secret_number) {
            Ordering::Less => result_text = "Too small, guess again:",
            Ordering::Greater => result_text = "Too big, guess again:",
            Ordering::Equal => result_text = {
                format!("You win! Attempts: {}.", attempts);
                break;
            }

        println!("\n{}", result_text);
    }
}
