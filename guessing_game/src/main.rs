extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 10);

  // println!("The secret number is: {}", secret_number);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    // immutable - let foo = 5;
    // mutable - let mut bar = 5;

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    // trim remove plain, For example 5\n
    // convert type from string to integer
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("You win!");
        break; // break loop
      }
    }
  }
}
