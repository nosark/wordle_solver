use rand::{thread_rng, Rng};
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

#[cfg(test)]
mod test {
    use super::grab_rand_word_from_dict;

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
}
