use rand::prelude::*;
use std::thread;
use std::time::Duration;
use colored::Colorize;

fn main(){
    let total_score = &mut 0;
    for round in 1..=10 {
        game_round(round, total_score);
    }
    clearscreen::clear().unwrap();
    println!("Game over!\n");
    let message = if *total_score == 0 {
        "Not likeminded afterall..."
    } else if *total_score < 10 {
        "A little likeminded."
    } else if *total_score < 20 {
        "Quite likeminded."
    } else if *total_score < 30 {
        "Super likeminded."
    } else {
        "Perfect score!"
    };
    println!("{} points earned collectively. {}\n", total_score.to_string(), message);
    
}

fn game_round(round: i32, total_score: &mut i32) {
    clearscreen::clear().unwrap();

    let starting_player = if round % 2 != 0 { "Player 1".bold().red() } else { "Player 2".bold().blue() };
    let guessing_player = if round % 2 == 0 { "Player 1".bold().red() } else { "Player 2".bold().blue() };

    if round == 1 {
        println!("{}", "Welcome to Likeminded!\n".bold());
    }
    
    let mut rng = rand::thread_rng();  
    let bonus: i32 = rng.gen_range(1..=6);

    if bonus == 1 {
        println!("Round {} of 10. {}\n", round.to_string(), "DOUBLE POINTS!!!".bold().purple());
    } else {
        println!("Round {} of 10.\n", round.to_string(),);
    }
    println!("Choose the guess word, {}!\n", starting_player);

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

    let target: i32 = rng.gen_range(0..=9);
    println!("Your target number is... {}\n", target);
    let filled_scale = format!("{} 0 {}{} 9 {}", lower_bound, "█".repeat((target * 2) as usize), "░".repeat(18 - ((target * 2) as usize)), upper_bound);
    let arrow = if round % 2 != 0 {
        format!("{}{}", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2), "▲".red())
    } else {
        format!("{}{}", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2), "▲".blue())
    };
    let label = format!("{}{}", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2), target.to_string());
    println!("{}\n{}\n{}\n", filled_scale, arrow, label);

    let mut guess_word = String::new();
    println!("What is the guess word?:");
    let _ = std::io::stdin().read_line(&mut guess_word).unwrap();
    guess_word = guess_word.trim().to_string();
    println!();

    clearscreen::clear().unwrap();
    
    println!("Make your guess, {}! The guess word is... {}\n", guessing_player, guess_word);
    let empty_scale = format!("{} 0 {} 9 {}", lower_bound, "░".repeat(18), upper_bound);
    println!("{}\n", empty_scale);
    println!("Enter your guess:");

    let guess: i32 = loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        match input.parse() {
            Ok(n) if n >= 0 && n <= 11 => break n,
            Ok(_) => println!("\nYour guess is outside of the scale. Try again..."),
            Err(_) => println!("\nYour guess was not a number. Try again..."),
        }
    };
    println!();

    let difference: i32 = target - guess;
    let mut score: i32 = 3 - difference.abs();
    if score < 0 {
        score = 0;
    }
    if bonus == 1 {
        score *= 2;
    }
    *total_score += score;

    for i in (1..=5).rev() {
        clearscreen::clear().unwrap(); 
        let message = if round < 4 { format!("{} points earned this round. Next round in", score) } else { format!("{} points earned this round. Game ending in", score) };
        if i > 1 {
            println!("{} {} seconds...\n", message, i.to_string());
        } else {
            println!("{} 1 second...\n", message);
        }
        let target_arrow = if round % 2 != 0 {
            format!("{}{}", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2), "▼".red())
        } else {
            format!("{}{}", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2), "▼".blue())
        };
        let target_label = format!("{}{}", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2), target.to_string());
        let guess_arrow = if round % 2 != 0 {
            format!("{}{}", " ".repeat(lower_bound.chars().count() + ((guess * 2) as usize) + 2), "▲".blue())
        } else {
            format!("{}{}", " ".repeat(lower_bound.chars().count() + ((guess * 2) as usize) + 2), "▲".red())
        };
        let guess_label = format!("{}{}", " ".repeat(lower_bound.chars().count() + ((guess * 2) as usize) + 2), guess.to_string());
        println!("{}\n{}\n{}\n{}\n{}\n", target_label, target_arrow, filled_scale, guess_arrow, guess_label);
        println!("{} points earned collectively.", total_score.to_string());
        thread::sleep(Duration::from_secs(1));
    }
}