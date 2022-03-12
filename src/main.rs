use std::error::Error;
use wordle_solver::wordle::Guesser;
use wordle_solver::wordle::Player;

fn main() -> Result<(), Box<dyn Error>> {
    let mut guesser = Guesser::new(false);
    Ok(())
}
