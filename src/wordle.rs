use colored::Colorize;
use std::collections::hash_map::Entry;
use std::error::Error;
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

/// This macro allows us to cleanly represent masks as
/// an array of initials / tokens that represent Correctness
/// enums without all the code clutter.
/// so instead of:
///     [Correctness::Correct, Correctness::Correct, Correctness::Correct, Correctness::Correct,
///         Correctness::Correct];
///
/// we get:
///
///     [C C C C C]
///
/// where both arrays are equal to each other, using the shorthand out of code
/// cleanliness and readability.
///
macro_rules! mask {
    (C) => { Correctness::Correct };
    (M) => { Correctness::Misplaced };
    (W) => { Correctness::Wrong };

    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}

impl Correctness {
    pub fn compute(answer: &str, guess: &str) -> [Correctness; 5] {
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

                    Entry::Vacant(_e) => {
                        continue;
                    }
                }
            }

            index += 1;
        }
        mask
    }

    pub fn display_mask_to_console(guess: &Box<Guess>) {
        for (c, l) in guess.mask.into_iter().zip(guess.word.chars()) {
            match c {
                Correctness::Correct => {
                    print!("{} ", l.to_string().green());
                }

                Correctness::Misplaced => {
                    print!("{} ", l.to_string().yellow());
                }
                Correctness::Wrong => {
                    print!("{} ", l.to_string().white())
                }
            }
        }
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

    fn rate_words_by_bits_of_info(&mut self, _dictionary: &'static str) -> HashSet<String, f64> {
        unimplemented!()
    }
}

impl Guesser for Agent {
    fn guess(&mut self, _answer: &'static str) -> Result<Box<Guess>, Box<dyn Error>> {
        unimplemented!()
    }

    fn rate_words_by_bits_of_info(&mut self, _dictionary: &'static str) -> HashSet<String, f64> {
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
        current_guess.mask = Correctness::compute(answer, current_guess.word.as_str());
        Correctness::display_mask_to_console(&current_guess);
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

        #[test]
        fn regular_guess() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
        }

        #[test]
        fn repeat_letters_guess() {
            assert_eq!(Correctness::compute("sally", "lllrr"), mask![M M W W W]);
        }

        #[test]
        fn all_misplaced() {
            assert_eq!(Correctness::compute("edcba", "acbde"), mask![M M M M M]);
        }

        #[test]
        fn two_correct_repeats_rest_misplace() {
            assert_eq!(Correctness::compute("aaedc", "aadce"), mask![C C M M M]);
        }

        #[test]
        fn all_wrong() {
            assert_eq!(Correctness::compute("abcde", "fhjki"), mask![W W W W W]);
        }

        #[test]
        fn correct_misplaced_every_other() {
            assert_eq!(Correctness::compute("abcde", "adcbe"), mask![C M C M C]);
        }

        #[test]
        fn four_correct_one_wrong() {
            assert_eq!(Correctness::compute("abcde", "abcdf"), mask![C C C C W]);
        }

        #[test]
        fn some_correct_some_misplaced() {
            assert_eq!(Correctness::compute("abcde", "abced"), mask![C C C M M]);
        }
    }
}
