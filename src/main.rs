mod state;

use colored::*;
use rand::seq::SliceRandom;
use state::{GameState, LetterCorrect};
use std::fs;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let selected_word = String::from(
        *fs::read_to_string("./dictionary.txt")?
            .split("\n")
            .collect::<Vec<&str>>()
            .choose(&mut rand::thread_rng())
            .expect("Error selecting random word"),
    );

    let mut state = GameState::new(selected_word);
    update_game(&state);

    loop {
        print!("Enter your guess: ");
        stdout().flush().expect("Cannot flush stdout");

        let mut guess = String::new();
        stdin().read_line(&mut guess)?;
        guess = guess.trim().to_string();

        if guess.len() != 5 {
            println!();
            println!("Guesses should be 5 characters long");
            continue;
        }

        state.add_guess(guess);
        update_game(&state);

        if state.game_over() {
            break;
        }
    }

    Ok(())
}

/// Update the game state to the console
fn update_game(state: &GameState) -> () {
    // clear the console
    print!("\x1B[2J");
    print!("\x1B[0;0f");

    println!("┏━━━━━━━━━━━━━━━━━┓");
    println!("┃   W O R D L E   ┃");
    println!("┣━━━━━━━━━━━━━━━━━┫");

    let mut lines = 0;

    // print each past guess with the correct colors
    for guess in state.past_guesses.iter() {
        if let Some(word) = guess {
            print!("┃    ");
            let formatted = format_with_color(word, state.validate_guess(word));
            formatted.iter().for_each(|c| print!("{} ", c));
            println!("   ┃");
            lines += 1;
        } else {
            break;
        }
    }

    // print the current guess with the correct colors
    if state.current_guess != "" {
        print!("┃    ");
        let formatted = format_with_color(&state.current_guess, state.validate_current());
        formatted.iter().for_each(|c| print!("{} ", c));
        println!("   ┃");
        lines += 1;
    }

    // add empty placeholders to represent the rest of the game board
    for _ in lines..state.past_guesses.len() + 1 {
        println!("┃                 ┃");
    }

    println!("┗━━━━━━━━━━━━━━━━━┛");

    if state.win() {
        println!("You found the hidden word!");
    } else if state.lose() {
        println!(
            "You could not find the hidden word in 6 tries. It was {}",
            state.word
        );
    }
}

/// Formats the given guess with the given correctness
/// Returns a vector of colored chatacters
fn format_with_color(word: &String, format: [LetterCorrect; 5]) -> Vec<ColoredString> {
    word.chars()
        .enumerate()
        .map(|(i, c)| match format[i] {
            Some(true) => c.to_string().green(),
            Some(false) => c.to_string().yellow(),
            None => c.to_string().normal(),
        })
        .collect()
}
