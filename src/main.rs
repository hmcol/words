use log::debug;
use log::info;
use std::fs; // import the debug macro from the log crate

mod words;

// -----------------------------------------------------------------------------

fn main() {
    colog::basic_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();
    info!("logging initialized");

    let wl = words::WordList::from_file("./lists/words_alpha.txt");

    wl.log_stats();

    let filtered_words = wl.find_scrabble_simple("least");

    // debug output filtered_words
    println!("found {} words", filtered_words.clone().len(),);

    for word in filtered_words {
        println!("{}", word);
    }
}

// predicates things i need:
// - P(word, letter) -> word contains the given letter
// - P(word, letters) -> word contains each of the given letters at least once
//                       letters with and without repeats perhaps
// - P(word, letters) -> word contains only the given letters

// for a given word, need these predicates:
// - has_letter(letter)
// - has_letters(letters)
// - made_of(letters) = word contains all the given letters and only those letters (with and without repeats) in other words, the word is a permutation/anagram of the given letters. this is good for scrabble and anagrams and anything where you have a specific bank of letters to use
