
/// Enum represents assigned score values to each letter
/// in relation to a guess. Each guess can have a score of :
/// Correct : The letter is a subset of the answer and in the correct position.
/// Misplaced: The letter is a subset of the answer, but in the wrong location.
/// Wrong: The letter is not a subset of the answer, nor is it in the correct position.
#[derive(Debug)]
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
        println!("im a player")
    }
}
impl Guesser for Agent {
    fn guess(&mut self, answer: &'static str) {
        println!("im a bot")
    }
}

impl PlayerFactory {
    pub fn new_player(is_agent: bool) -> Box<dyn Guesser> {
        match is_agent {
            false => Box::new(Player {wins: 0, losses: 0}),
            true => Box::new(Agent {wins: 0, losses: 0})
        }
    }
}

pub trait Guesser {
    fn guess(&mut self, answer: &'static str);
}

/// The play function allows the user to attempt six guesses to
/// try and guess the answer the game has selected from the dictionary.
/// the guesser can be a real user or an agent that plays the game itself.
pub fn play<G: Guesser>(answer: &'static str, guesser: G) {
    let guesses = 0;
    loop {
        if guesses == 6 {
            break;
        }
    }
    println!("Thank you for playing!");
}
