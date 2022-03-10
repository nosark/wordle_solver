pub mod wordle {

    /// Enum represents assigned score values to each letter
    /// in relation to a guess. Each guess can have a score of :
    /// Correct : The letter is a subset of the answer and in the correct position.
    /// Misplaced: The letter is a subset of the answer, but in the wrong location.
    /// Wrong: The letter is not a subset of the answer, nor is it in the correct position.
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
    pub struct GuessHistory {
        guesses: Vec<Guess>,
    }

    /// A Guess represents an attempt to match a word to the secret
    /// answer in a game of wordle.
    pub struct Guess {
        /// A guess attempt being made by the player.
        word: String,
        /// A mask masks the word for each guess, color coding
        /// each letter to represent some form of Correctness (ie: Correct, Misplaced, Wrong)
        /// in accordance to the letter of each word being a subset of the answer, and whether
        /// or not the letter is in its correct position relative to the answer.
        mask: [Correctness; 5],
    }

    /// The Player struct can be utilized as an actual human player,
    /// or you can use it to implement an AI agent of your choosing.
    pub struct Player {
        wins: i32,
        losses: i32,
    }

    pub struct Agent {
        wins: i32,
        losses: i32,
        //algorithm: todo!(),
    }

    impl Guesser for Player {}
    impl Guesser for Agent {}

    pub trait Guesser {
        // fn new(is_agent: bool) -> Box<dyn Guesser> where Self: Sized {
        //     match is_agent {
        //         true => Box::new(Agent{wins: 0, losses: 0}),
        //         false => Box::new(Player{wins: 0, losses: 0}),
        //     }
        // }

        fn guess(&mut self, answer: &'static str) {
            unimplemented!()
        }
    }

    /// The play function allows the user to attempt six guesses to
    /// try and guess the answer the game has selected from the dictionary.
    /// the guesser can be a real user or an agent that plays the game itself.
    pub fn play<G: Guesser>(answer: &'static str, guesser: G) {
        unimplemented!()
    }
}
