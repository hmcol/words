use crate::filter::WordFilter;

#[derive(Debug)]
pub struct WordFinder {
    pub file_path: String,
    pub word_list: Vec<String>,
    pub filters: Vec<WordFilter>,
}

impl Default for WordFinder {
    fn default() -> Self {
        Self::from_file("./lists/dictionary.txt")
    }
}

impl WordFinder {
    pub fn from_file(file_path: &str) -> WordFinder {
        let file = std::fs::read_to_string(file_path).expect("failed to read file");

        let words = file
            .split_whitespace() // assume one word per line
            .map(|w| w.to_string().to_lowercase()) // convert to lowercase
            .filter(|w| w.chars().all(|c| c.is_alphabetic())) // only alphabetic
            .collect::<Vec<String>>();

        // let mut info_string = format!("found {} words in file {}", words.len(), file_path);

        // for word in words.iter().take(10) {
        //     info_string.push_str(&format!("\n{:?}", word));
        // }

        // info_string.push_str("\n...");

        // log::info!("{}", info_string);

        WordFinder {
            file_path: file_path.to_string(),
            word_list: words,
            filters: Vec::new(),
        }
    }

    // pub fn log_stats(&self) {
    //     let word_lengths: Vec<usize> = self.word_list.iter().map(|word| word.len()).collect();

    //     let mut length_counts = std::collections::HashMap::new();

    //     for length in word_lengths.iter() {
    //         let count = length_counts.entry(length).or_insert(0);
    //         *count += 1;
    //     }

    //     for (length, count) in length_counts.iter() {
    //         log::info!("{:02}: {}", length, count);
    //     }
    // }

    // -------------------------------------------------------------------------

    pub fn add_filter(&mut self, filter: WordFilter) {
        self.filters.push(filter);
    }

    pub fn add_filter_idx(&mut self, index: usize) {
        if let Some(f) = WordFilter::from_index(index) {
            self.filters.push(f);
        }
    }

    // pub fn get_filtered_words(&self) -> Vec<&String> {
    //     self.word_list
    //         .iter()
    //         .filter(|word| self.filters.iter().all(|f| f.matches(word)))
    //         .collect()
    // }

    pub fn iter_filtered_words(&self) -> impl Iterator<Item = &String> {
        self.word_list
            .iter()
            .filter(move |word| self.filters.iter().all(|f| f.matches(word)))
    }
}
