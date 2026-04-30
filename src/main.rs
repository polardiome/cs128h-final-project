use rand::prelude::*;
use std::thread;
use std::time::Duration;

fn main(){
    let mut p1_score = &mut 0;
    let mut p2_score = &mut 0;
    for round in 1..=4 {
        game_round(round, p1_score, p2_score);
    }
    clearscreen::clear().unwrap();
    println!("Game over!\n");
    println!("Player 1 earned {} points. Player 2 earned {} points.\n", p1_score.to_string(), p2_score.to_string());
    if p1_score > p2_score {
        println!("Player 1 wins!");
    } else if p2_score > p1_score {
        println!("Player 2 wins!");
    } else {
        println!("It's a tie!");
    }
}

fn game_round(round: i32, p1_score: &mut i32, p2_score: &mut i32) {
    clearscreen::clear().unwrap();

    let starting_player = if round % 2 != 0 { "Player 1" } else { "Player 2" };
    let guessing_player = if round % 2 == 0 { "Player 1" } else { "Player 2" };

    if round == 1 {
        println!("Welcome to Likeminded!\n");
    }
    println!("Round {} of 10. {} chooses the guess word!\n", round.to_string(), starting_player);

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
    let target: i32 = rng.gen_range(0..=9);
    println!("Your target number is... {}\n", target);
    let filled_scale = format!("{} 0 {}{} 9 {}", lower_bound, "█".repeat((target * 2) as usize), "░".repeat(18 - ((target * 2) as usize)), upper_bound);
    let arrow = format!("{}▲", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2));
    let label = format!("{}{}", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2), target.to_string());
    println!("{}\n{}\n{}\n", filled_scale, arrow, label);

    let mut guess_word = String::new();
    println!("What is the guess word?:");
    let _ = std::io::stdin().read_line(&mut guess_word).unwrap();
    guess_word = guess_word.trim().to_string();
    println!();

    clearscreen::clear().unwrap();
    
    println!("{} guesses! The guess word is... {}\n", guessing_player, guess_word);
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

    if round % 2 == 0 { *p1_score += score; } else { *p2_score += score; }

    for i in (1..=5).rev() {
        clearscreen::clear().unwrap(); 
        let message = if (round < 4) { format!("{} earns {} points. Next round in", guessing_player, score) } else { format!("{} earns {} points. Game ending in", guessing_player, score) };
        if i > 1 {
            println!("{} {} seconds...\n", message, i.to_string());
        } else {
            println!("{} 1 second...\n", message);
        }
        let target_arrow = format!("{}▼", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2));
        let target_label = format!("{}{}", " ".repeat(lower_bound.chars().count() + ((target * 2) as usize) + 2), target.to_string());
        let guess_arrow = format!("{}▲", " ".repeat(lower_bound.chars().count() + ((guess * 2) as usize) + 2));
        let guess_label = format!("{}{}", " ".repeat(lower_bound.chars().count() + ((guess * 2) as usize) + 2), guess.to_string());
        println!("{}\n{}\n{}\n{}\n{}\n", target_label, target_arrow, filled_scale, guess_arrow, guess_label);
        println!("Player 1 has {} points. Player 2 has {} points.\n", p1_score.to_string(), p2_score.to_string());
        thread::sleep(Duration::from_secs(1));
    }
}