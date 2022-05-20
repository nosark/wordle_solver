#[allow(dead_code, unused)]
use colored::Colorize;
use std::collections::hash_map::Entry;
use std::error::Error;
use std::{vec, string};
use std::{collections::HashMap, collections::HashSet, io::stdin};
/// Enum represents assigned score values to each letter
/// in relation to a guess. Each guess can have a score of :
/// Correct : The letter is a subset of the answer and in the correct position.
/// Misplaced: The letter is a subset of the answer, but in the wrong location.
/// Wrong: The letter is not a subset of the answer, nor is it in the correct position.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Correctness {
    /// Masks the letter as Green
    Correct,
    /// Masks the letter as Yellow
    Misplaced,
    /// Masks the letter as Gray
    Wrong,
}

/*
macro_rules! expect_mask! {
    ($a:expr, $b:expr) => {
        
    };
} */
impl Correctness {
    pub fn compute(answer: String, guess: String) -> [Correctness;5] {
        let mut mask = [Correctness::Wrong; 5];
        let mut index = 0;
        let mut string_histogram = HashMap::<char, i32>::new();
        for i in answer.chars() {
            match string_histogram.entry(i) {
                Entry::Occupied(mut e) => {
                
                    *e.get_mut() += 1;
                }

                Entry::Vacant(e) => {
                    e.insert(1);
                }
            }
        }



        for (i, j) in answer.chars().zip(guess.chars()) {
            if i == j && *string_histogram.get(&j).unwrap() > 0 {               
                mask[index as usize] = Correctness::Correct;
            } else {
                match string_histogram.entry(j) {
                    Entry::Occupied(mut e) => {
                        if *e.get() > 0 {
                            mask[index as usize] = Correctness::Misplaced;
                            *e.get_mut() -= 1;
                        }
                    }

                    Entry::Vacant(_e) => { continue; }
                }
            }

            index += 1;
        }
        mask
    }
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
    pub fn new(word: String, mask: [Correctness; 5]) -> Self {
        Guess { word, mask }
    }
}

#[derive(Debug)]
pub enum PlayerType {
    Human(Player),
    Bot(Agent),
}

/// The Player struct can be utilized as an actual human player,
/// or you can use it to implement an AI agent of your choosing.
#[derive(Debug, Clone, Copy)]
pub struct Player {}

#[derive(Debug, Clone, Copy)]
pub struct Agent {}

#[derive(Debug, Clone, Copy)]
pub struct PlayerFactory {}

macro_rules! get_input {
    ($i:ident) => {
        stdin()
            .read_line(&mut $i)
            .ok()
            .expect("failed to read input");
    };
}

impl Guesser for Player {
    fn guess(&mut self, _answer: &'static str) -> Result<Box<Guess>, Box<dyn Error>> {
        let mut _correct_vec = [Correctness::Wrong; 5];
        let mut user_input = String::new();
        get_input!(user_input);
        let guess_value = Box::new(Guess {
            word: user_input.to_owned(),
            mask: _correct_vec.to_owned(),
        });
        Ok(guess_value)
    }
    
    fn rate_words_by_bits_of_info(&mut self, dictionary: &'static str) -> HashSet<String, f64> {
        unimplemented!()
    }
}

pub fn evaluate<'a>(mut guess: &'a mut Box<Guess>, answer: &'static str) -> &'a [Correctness; 5] {
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

    //evaluate guess
    let mut guess_pos: i8 = 0;
    for i in guess.word.chars() {
        if guess_pos == 5 {
            break;
        }
        match string_lookup.entry(i) {
            Entry::Occupied(mut e) => {
                // check the position so we can populate mask correctness
                let vec_positions = e.get_mut();
                if vec_positions.is_empty() {
                    guess.mask[guess_pos as usize] = Correctness::Wrong;
                    break;
                }
                for i in 0..vec_positions.len() {
                    if vec_positions[i] as i8 == guess_pos {
                        guess.mask[guess_pos as usize] = Correctness::Correct;
                        vec_positions.swap_remove(i);
                    } else {
                        // none of the logged positions were equal, so partial correct
                        guess.mask[guess_pos as usize] = Correctness::Misplaced;
                    }
                }
            }

            Entry::Vacant(_e) => {
                // mark mask as wrong
                guess.mask[guess_pos as usize] = Correctness::Wrong;
            }
        }
        guess_pos += 1;
    }

    &guess.mask
}

pub fn display_answer_correctness_to_console(guess: &Box<Guess>) {
    // Color the letters and display to user.
    let mut i: i8 = 0;
    for c in guess.word.chars() {
        if i == 5 {
            break;
        }
        match guess.mask[i as usize] {
            Correctness::Correct => print!("{} ", c.to_string().green()),
            Correctness::Misplaced => print!("{} ", c.to_string().yellow()),
            Correctness::Wrong => print!("{} ", c.to_string().white()),
        }
        i += 1;
    }

    // Displays correctness mask. delete later.
    println!("{:?}", guess.mask);
}

impl Guesser for Agent {
    fn guess(&mut self, answer: &'static str) -> Result<Box<Guess>, Box<dyn Error>> {
        unimplemented!()
    }

    fn rate_words_by_bits_of_info(&mut self, dictionary: &'static str) -> HashSet<String, f64> {
        unimplemented!()
    }
}

impl PlayerFactory {
    pub fn new_player(is_agent: bool) -> Box<dyn Guesser> {
        match is_agent {
            false => Box::new(Player {}),
            true => Box::new(Agent {}),
        }
    }
}

pub trait Guesser {
    fn guess(&mut self, answer: &'static str) -> Result<Box<Guess>, Box<dyn Error>>;
    fn rate_words_by_bits_of_info(&mut self, dictionary: &'static str) -> HashSet<String, f64>;
}

/// The play function allows the user to attempt six guesses to
/// try and guess the answer the game has selected from the dictionary.
/// the guesser can be a real user or an agent that plays the game itself.
pub fn play(answer: &'static str, mut guesser: Box<dyn Guesser>) {
    let mut guesses = 0;
    let mut guess_history = GuessHistory {
        guesses: Vec::<Guess>::new(),
    };
    println!(
        "
        This is Not WOrdle, where you will have 6 attempts to guess\n
        the secret word. You are only allowed to guess words in this 5\n
        letter word dictionary. If you get a letter correct, but out of position\n
        it will be marked in yellow, if you get the letter and position correct\n
        in green, and white if the letter is completely incorrect.\n

        Good Luck and Start Guessing!
    "
    );

    loop {
        if guesses == 6 {
            println!("the word was: {}", answer);
            break;
        }
        let mut current_guess = guesser.guess(answer).expect("failed to construct guess");
        evaluate(&mut current_guess, answer);
        display_answer_correctness_to_console(&current_guess);
        guess_history.guesses.push(*current_guess);
        guesses += 1;
    }
    println!("Thank you for playing!");
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod correctness {

    use crate::wordle::Correctness;
    use crate::wordle::Guess;

        #[test]
        fn regular_guess() {
            let mut guess = Guess {word: String::from("abcde"), mask: [Correctness::Wrong; 5]};
            guess.mask = Correctness::compute(guess.word,  String::from("abcde"));                                 
            assert_eq!(guess.mask, [Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct]);
        }

        #[test]
        fn repeat_letters_guess() {
            let mut guess = Guess {word: String::from("lllrr"), mask: [Correctness::Wrong; 5]};
            guess.mask = Correctness::compute(String::from("sally"), guess.word);
            assert_eq!(guess.mask, [Correctness::Misplaced, Correctness::Misplaced, Correctness::Wrong, Correctness::Wrong, Correctness::Wrong]);
        }

    }
}
