use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

// This will be the dictionary used to play the game
// not only to retrieve a word that becomes our "answer"
// but to also guarentee the user can't waste guesses on
// irrelevant words.
//const WORDS: &str = include_str!("../res/dictionary.txt");

// Simple utility function.
// used this function to strip unwanted numbers from original dictionary file.
#[allow(dead_code)]
fn filter_text_dict(file_path: &Path, new_file_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(new_file_path)
        .unwrap();
    let file_results: String = fs::read_to_string(file_path)?.parse()?;
    for words in file_results.split_whitespace() {
        let trimmed_word = words.trim();
        if count % 2 == 0 {
            writeln!(file, "{}", trimmed_word).unwrap();
        }
        count += 1;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_filtering = filter_text_dict(
        Path::new("./res/dictionary.txt"),
        Path::new("./res/answers.txt"),
    );
    match file_filtering {
        Err(e) => panic!("file filtering failed! {}", e),
        _ => (),
    }
    Ok(())
}
