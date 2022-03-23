use std::{vec};
use std::{io::stdin, collections::HashMap};
use std::collections::hash_map::Entry;
use colored::Colorize;
/// Enum represents assigned score values to each letter
/// in relation to a guess. Each guess can have a score of :
/// Correct : The letter is a subset of the answer and in the correct position.
/// Misplaced: The letter is a subset of the answer, but in the wrong location.
/// Wrong: The letter is not a subset of the answer, nor is it in the correct position.
#[derive(Debug, Copy, Clone)]
pub enum Correctness {
    /// Masks the letter as Green
    Correct,
    /// Masks the letter as Yellow
    Misplaced,
    /// Masks the letter as Gray
    Wrong,
}

///  The GuessHistory struct keeps track of
/// our previous guesses made for the lifetime of the game/round.
#[derive(Debug)]
pub struct GuessHistory {
    guesses: Vec<Guess>,
}

/// A Guess represents an attempt to match a word to the secret
/// answer in a game of wordle.
#[derive(Debug)]
pub struct Guess {
    /// A guess attempt being made by the player.
    word: String,
    /// A mask masks the word for each guess, color coding
    /// each letter to represent some form of Correctness (ie: Correct, Misplaced, Wrong)
    /// in accordance to the letter of each word being a subset of the answer, and whether
    /// or not the letter is in its correct position relative to the answer.
    mask: [Correctness; 5],
}

impl Guess {
    pub fn new(word: String, mask:[Correctness; 5]) -> Self {
        Guess{ word, mask }
    }
}

#[derive(Debug)]
pub enum PlayerType {
    Human(Player),
    Bot(Agent),
}

/// The Player struct can be utilized as an actual human player,
/// or you can use it to implement an AI agent of your choosing.
#[derive(Debug)]
pub struct Player {
    wins: i32,
    losses: i32,
}

#[derive(Debug)]
pub struct Agent {
    wins: i32,
    losses: i32,
    //algorithm: todo!(),
}

#[derive(Debug)]
pub struct PlayerFactory {}

impl Guesser for Player {
    fn guess(&mut self, answer: &'static str) {

        let mut pos = 0;
        let mut string_lookup = HashMap::<char, Vec<i32>>::new();
        for x in answer.chars() {
            match string_lookup.entry(x) {
                Entry::Vacant(e) => { 
                    e.insert(vec![pos]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(pos);
                }
            }
            pos += 1;
        }

        println!("Please submit a 5 letter word: ");
        let mut correct_vec = [Correctness::Wrong;  5];
        let mut user_input = String::new();
        stdin().read_line(&mut user_input)
            .ok()
            .expect("failed to read input");

        let mut guess_pos = 0;
        for i in user_input.chars() {
            if guess_pos == 5 {
                break;
            }
            match string_lookup.entry(i) {
                
                Entry::Occupied(e) => {
                    // check the position so we can populate mask correctness
                    let vec_positions = e.get();
                    for i in 0..vec_positions.len() {
                        if vec_positions[i] as i32 == guess_pos {
                            correct_vec[guess_pos as usize] = Correctness::Correct;
                        } else {
                            // none of the logged positions were equal, so partial correct
                            correct_vec[guess_pos as usize] = Correctness::Misplaced;
                        }
                    }
                }

                Entry::Vacant(_e) => {
                    // mark mask as wrong
                    correct_vec[guess_pos as usize] = Correctness::Wrong;
                }
            }
            guess_pos += 1;
        }

        // Color the letters and display to user.
        let mut i = 0;
        for c in user_input.chars() {
            if i == 5 {
                break;
            }
            match correct_vec[i as usize] {
                Correctness::Correct => print!("{} ", c.to_string().green()),
                Correctness::Misplaced => print!("{} ", c.to_string().yellow()),
                Correctness::Wrong => print!("{} ", c.to_string().white())
            }
            i += 1;
        }

        // Displays correctness mask. delete later.
        println!("{:?}", correct_vec);

    }

}

impl Guesser for Agent {
    #[allow(dead_code, unused)]
    fn guess(&mut self, answer: &'static str) {
        println!("im a bot")
    }
}

impl PlayerFactory {
    pub fn new_player(is_agent: bool) -> Box<dyn Guesser> {
        match is_agent {
            false => Box::new(Player { wins: 0, losses: 0 }),
            true => Box::new(Agent { wins: 0, losses: 0 }),
        }
    }
}

pub trait Guesser {
    fn guess(&mut self, answer: &'static str);
}

/// The play function allows the user to attempt six guesses to
/// try and guess the answer the game has selected from the dictionary.
/// the guesser can be a real user or an agent that plays the game itself.
pub fn play(answer: &'static str, mut guesser: Box<dyn Guesser>) {
    let mut guesses = 0;
    loop {
        if guesses == 6 {
            println!("the word was: {}", answer);
            break;
        }
        guesser.guess(answer);
        guesses += 1;
    }
    println!("Thank you for playing!");
}


#[cfg(test)]
mod test {
    
}