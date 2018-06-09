extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number game! Input your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.\n");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number.\n");

    let comparison_text;

    let secret_number = rand::thread_rng().gen_range(0, 100);

    match guess.cmp(&secret_number) {
        Ordering::Less => comparison_text = "Too small",
        Ordering::Greater => comparison_text = "Too big",
        Ordering::Equal => comparison_text = "You win",
    }

    println!("\n{}, the secret was: {}. You guessed: {}.\n", comparison_text, secret_number, guess);
}
