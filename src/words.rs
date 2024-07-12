pub struct WordList {
    pub words: Vec<String>,
}

impl WordList {
    pub fn from_file(file_path: &str) -> WordList {
        let file = std::fs::read_to_string(file_path).expect("failed to read file");

        let words = file
            .split_whitespace()                                 // assume one word per line
            .map(|w| w.to_string().to_lowercase())              // convert to lowercase
            .filter(|w| w.chars().all(|c| c.is_alphabetic()))   // only alphabetic
            .collect::<Vec<String>>();

        let mut info_string = format!("found {} words in file {}", words.len(), file_path);

        for word in words.iter().take(10) {
            info_string.push_str(&format!("\n{:?}", word));
        }

        info_string.push_str("\n...");

        log::info!("{}", info_string);

        WordList { words }
    }

    pub fn log_stats(&self) {
        let word_lengths: Vec<usize> = self.words.iter().map(|word| word.len()).collect();

        let mut length_counts = std::collections::HashMap::new();

        for length in word_lengths.iter() {
            let count = length_counts.entry(length).or_insert(0);
            *count += 1;
        }

        for (length, count) in length_counts.iter() {
            log::info!("{:02}: {}", length, count);
        }
    }

    // -------------------------------------------------------------------------

    pub fn find(&self, predicate: impl Fn(&String) -> bool) -> Vec<&String> {
        self.words.iter().filter(|word| predicate(word)).collect()
    }

    /// Find words that are spelled using only the given letters, repeat letters allowed
    ///
    /// given `letters` a "set" of letters, we only require that each letter of the word is in `letters`
    pub fn find_with_only_letters(&self, letters: &str) -> Vec<&String> {
        let is_valid = move |word: &String| -> bool {
            for l in word.chars() {
                if !letters.contains(l) {
                    return false;
                }
            }
            true
        };

        self.words.iter().filter(|word| is_valid(word)).collect()
    }

    /// Find words that could be played given a set of tiles in a scrabble game
    ///
    /// `tiles` must be a string containing only letters and possibly question marks to represent blank tiles
    pub fn find_scrabble_simple(&self, tiles: &str) -> Vec<&String> {
        let is_valid = move |word: &String| -> bool {
            let mut tiles = tiles.to_string();

            for l in word.chars() {
                if let Some(pos) = tiles.find(l) {
                    tiles.remove(pos);
                } else if let Some(pos) = tiles.find('?') {
                    tiles.remove(pos);
                } else {
                    return false;
                }
            }

            true
        };

        self.words.iter().filter(|word| is_valid(word)).collect()
    }

    pub fn find_starts_with(&self, prefix: &str) -> Vec<&String> {
        self.words
            .iter()
            .filter(|word| word.starts_with(prefix))
            .collect()
    }

    pub fn find_ends_with(&self, suffix: &str) -> Vec<&String> {
        self.words
            .iter()
            .filter(|word| word.ends_with(suffix))
            .collect()
    }

    pub fn find_contains(&self, substring: &str) -> Vec<&String> {
        self.words
            .iter()
            .filter(|word| word.contains(substring))
            .collect()
    }

    pub fn find_length(&self, length: usize) -> Vec<&String> {
        self.words
            .iter()
            .filter(|word| word.len() == length)
            .collect()
    }
}
