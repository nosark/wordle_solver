use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use rand::{thread_rng, Rng};

// Simple utility function.
// used this function to strip unwanted numbers from original dictionary file.
#[allow(dead_code)]
pub fn filter_text_from_file(
    file_path: &Path,
    new_file_path: &Path,
) -> Result<(), Box<dyn Error>> {
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

pub fn grab_rand_word_as_slice(dictionary: &str) -> &str {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(0..dictionary.len());
    let rand_remainder = rand_num % 5;
    let diff = rand_num - rand_remainder;
    let test_slice = &dictionary[diff..diff+5];
    test_slice
}
