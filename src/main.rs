use std::io;
fn main() {
    println!("Input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("your guess is {guess} /b,isn't it ? {}", guess);
}
