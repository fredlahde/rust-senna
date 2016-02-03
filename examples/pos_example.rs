//! A simple example how you can use `rustsenna` for
//! part-of-speech tagging of sentences.

extern crate rustsenna;

use rustsenna::sennapath::SENNA_PATH;
use rustsenna::senna::{Senna, ParseOption};

fn main() {
    let mut senna = Senna::new(SENNA_PATH.to_owned());
    let sentence = senna.parse("This is not a sentence.", ParseOption::GeneratePOS);
    for word in sentence.get_words() {
        print!("\"{}\" - {}\n", word.get_string(), word.get_pos().to_str());
    }
}

