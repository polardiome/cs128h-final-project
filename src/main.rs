use rand::prelude::*;

fn main(){
    println!("Welcome to Likeminded! Player 1 begins.");

    let mut lower_bound = String::new();
    println!("Describe the lower bound:");
    let _ = std::io::stdin().read_line(&mut lower_bound).unwrap();
    lower_bound = lower_bound.trim().to_string();
    println!();

   let mut upper_bound = String::new();
    println!("Describe the upper bound:");
    let _ = std::io::stdin().read_line(&mut upper_bound).unwrap();
    upper_bound = upper_bound.trim().to_string();
    println!();

    let mut rng = rand::thread_rng();
    let target = rng.gen_range(0..12);
    println!("Your target number is... {}", target);
    println!();

    let mut guess_word = String::new();
    println!("What is the guess word?:");
    let _ = std::io::stdin().read_line(&mut guess_word).unwrap();
    guess_word= guess_word.trim().to_string();
    println!();

    clearscreen::clear().unwrap();

    let guess: i32 = loop {
        let mut input = String::new();
        println!("Player 2, make your guess:");
        let _ = std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        match input.parse() {
            Ok(n) if n >= 0 && n <= 11 => break n,
            Ok(_) => println!("Uh oh! Your guess is off of the scale!"),
            Err(_) => println!("🫪 Your input wasn't a number. Try again..."),
        }
    };
    println!();

    let difference: i32 = target - guess;
    let mut score: i32 = 3 - difference.abs();
    if score < 0 {
        score = 0;
    }
    println!("Your score is {}", score);
}