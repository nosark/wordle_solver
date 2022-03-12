use std::error::Error;
use wordle_solver::utility::grab_rand_word_as_slice;
use wordle_solver::wordle::Guesser;
use wordle_solver::wordle::Player;
use wordle_solver::wordle::PlayerFactory;

/// This will be the dictionary used to play the game
/// not only to retrieve a word that becomes our "answer"
/// but to also guarentee the user can't waste guesses on
/// irrelevant words.
const WORDS: &str = include_str!("../res/dictionary.txt");

fn main() -> Result<(), Box<dyn Error>> {
    //TODO: implement the game now!\
    
    let random_word = grab_rand_word_as_slice(&WORDS);
    let mut guesser = PlayerFactory::new_player(false);
    //wordle_solver::wordle::play(random_word, guesser);
    // guesser.guess("lol");
    Ok(())
}
