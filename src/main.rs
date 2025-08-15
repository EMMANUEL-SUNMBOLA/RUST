use std::io;

use rand::Rng;
fn main() {

    let secret = rand::thread_rng().gen_range(1..=100);
    println!("Input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("your guess is {guess} but the secret number is {secret}");
}
