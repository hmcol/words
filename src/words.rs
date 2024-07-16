use crate::{pred::Predicate, sort::Sorter};

#[derive(Debug)]
pub struct WordFinder {
    pub file_path: String,
    pub word_list: Vec<String>,
    pub predicates: Vec<Predicate>,
    pub sorter: Sorter,
}

impl Default for WordFinder {
    fn default() -> Self {
        let mut wf = Self::from_file("./lists/dictionary.txt");
        wf.predicates.push(crate::pred::Predicate::EndsWith("ing".to_string()));
        wf
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
            ..Default::default()
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

    pub fn add_predicate(&mut self, index: usize) {
        if let Some(p) = Predicate::from_index(index) {
            self.predicates.push(p);
        }
    }

    pub fn remove_predicate(&mut self, index: usize) {
        self.predicates.remove(index);
    }

    // pub fn get_filtered_words(&self) -> Vec<&String> {
    //     self.word_list
    //         .iter()
    //         .filter(|word| self.filters.iter().all(|f| f.matches(word)))
    //         .collect()
    // }

    pub fn iter_filtered(&self) -> impl Iterator<Item = &String> {
        self.word_list
            .iter()
            .filter(move |word| self.predicates.iter().all(|f| f.matches(word)))
    }

    pub fn get_filtered_sorted(&self) -> Vec<&String> {
        let mut word_vec: Vec<&String> = self.iter_filtered().collect();
        word_vec.sort_by(|left, right| self.sorter.cmp(left, right));
        word_vec
    }
}
