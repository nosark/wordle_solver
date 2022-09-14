use crate::wordle::{Correctness, Guess};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

// Simple utility function.
// used this function to strip unwanted numbers from original dictionary file.
#[allow(dead_code)]
pub fn filter_text_from_file(file_path: &Path, new_file_path: &Path) -> Result<(), Box<dyn Error>> {
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

/// Grabs a slice from a &str, randomly selecting a word from
/// the dictionary.
pub fn grab_rand_word_from_dict(dictionary: &str) -> &str {
    let words: Vec<&str> = dictionary.split_whitespace().collect();
    let rand_num = thread_rng().gen_range(0..words.len());
    words[rand_num]
}

pub fn filter_possible_answers(guess: &Box<Guess>, dictionary: &'static str) -> Vec<&'static str> {
    // Cases to remove
    // 1. str1 and str2 chars dont match and guess mask marked Correct
    // 2. str1 and str2 match and guess mask marked Wrong
    // 3. str2 (string in dictionary) does not have all misplaced chars
    // 4. str2 (string in dictionary) is missing all Correct chars
    // 5. TODO: need to account for misplaced letters and how to filter
    // out words that don't contain the proper amount correctly.
    let mut potential_answers = Vec::<&'static str>::new();
    let words: Vec<&'static str> = dictionary.split_whitespace().collect();
    for word in words {
        let is_possible_answer = is_potential_answer(guess, &word);
        match is_possible_answer {
            true => {
                potential_answers.push(word);
            }

            false => continue,
        }
    }

    potential_answers
}

pub fn is_potential_answer(guess: &Box<Guess>, current_word: &str) -> bool {
    let dict_word = current_word.as_bytes();
    let guess_word = guess.word.as_bytes();

    // Pre process current word for misplaced character lookups.
    let mut str_histogram = HashMap::<u8, i32>::new();
    for c in dict_word {
        *str_histogram.entry(*c).or_insert(1) += 1;
    }

    //Now check for Wrong characters that match and correct chacters that don't exist'
    //Otherwise we look for misplaced characters before we clear the word for possible answers.
    for i in 0..5 {
        if guess.mask[i] == Correctness::Correct && guess_word[i] != dict_word[i] {
            return false;
        } else if guess.mask[i] == Correctness::Wrong && guess_word[i] == dict_word[i] {
            return false;
        } else {
            if let Some(key) = str_histogram.get_mut(&guess_word[i]) {
                if *key <= 0 {
                    return false;
                }

                *key -= 1;
            }
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::filter_possible_answers;
    use super::grab_rand_word_from_dict;
    use crate::mask;
    use crate::wordle::Correctness;
    use crate::wordle::Guess;
    const WORDS: &str = include_str!("../res/dictionary.txt");
    const TEST_WORDS: &str = include_str!("../res/tests/slice_grab_test.txt");

    #[test]
    pub fn grab_random_word_as_slice() {
        unimplemented!()
    }

    #[test]
    pub fn grab_random_word_vec() {
        let words: Vec<&str> = TEST_WORDS.split_whitespace().collect();

        let sample_word = grab_rand_word_from_dict(&TEST_WORDS);
        let mut words_match = [""; 2];
        for &word in &words {
            println!("{}", word);
            if word.trim().eq(sample_word.trim()) {
                words_match = [&word, &sample_word];
            }
        }
        assert_eq!(words_match, [sample_word, sample_word]);
    }

    #[test]
    pub fn filter_out_possible_answer_test_basic() {
        let test_guess = Box::new(Guess {
            word: String::from("words"),
            mask: mask![C C C W W],
            is_correct: false,
        });
        let results = filter_possible_answers(&test_guess, TEST_WORDS);
        assert_eq!(results, ["worry"]);
    }

    #[test]
    pub fn filter_out_possible_answers_mask_all_wrong() {
        let test_guess = Box::new(Guess {
            word: String::from("xxxxx"),
            mask: mask![W W W W W],
            is_correct: false,
        });

        let results = filter_possible_answers(&test_guess, TEST_WORDS);
        assert_eq!(results, ["toons", "barks", "worry"]);
    }

    #[test]
    pub fn filter_possible_answers_correct_guess() {
        let test_guess = Box::new(Guess {
            word: String::from("barks"),
            mask: mask![C C C C C],
            is_correct: true,
        });

        let results = filter_possible_answers(&test_guess, TEST_WORDS);
        assert_eq!(results, ["barks"]);
    }

    #[test]
    pub fn filter_possible_answers_all_misplaced() {
        let test_guess = Box::new(Guess {
            word: String::from("abksr"),
            mask: mask![M M M M M],
            is_correct: false,
        });

        let results = filter_possible_answers(&test_guess, TEST_WORDS);
        assert_eq!(results, ["barks"]);
    }
}
