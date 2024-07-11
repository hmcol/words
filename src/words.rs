pub struct WordList {
    pub words: Vec<String>,
}

impl WordList {
    pub fn from_file(file_path: &str) -> WordList {
        let file = std::fs::read_to_string(file_path).expect("failed to read file");

        let words = file
            .split_whitespace()
            .map(|w| w.to_string())
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
}
