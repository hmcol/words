use crate::{
    ord::{WordOrder, ORDER_NAMES},
    pred::{WordPredicate, PREDICATE_NAMES},
};

#[derive(Debug)]
pub struct WordFinder {
    pub file_path: String,
    pub word_list: Vec<String>,
    pub predicates: Vec<WordPredicate>,
    pub word_order: WordOrder,
}

/// this can just be derived, but i want the default word list until i make it user editable and
/// at least one good predicate until its faster with bigger word lists
impl Default for WordFinder {
    fn default() -> Self {
        let mut wf = Self {
            file_path: String::new(),
            word_list: Vec::new(),
            predicates: Vec::new(),
            word_order: WordOrder::default(),
        };

        wf.load_file("./lists/words.txt");
        wf.add_predicate(2); // EndsWith
        wf.update_predicate(0, "ing");

        wf
    }
}

impl WordFinder {
    pub fn load_file(&mut self, file_path: &str) {
        // do something better here for error handling, just don't want to crash rn
        if let Ok(file) = std::fs::read_to_string(file_path) {
            self.file_path = file_path.to_string();
            self.word_list = file
                .split_whitespace() // assume one word per line
                .map(|w| w.to_string().to_lowercase()) // convert to lowercase
                .filter(|w| w.chars().all(|c| c.is_alphabetic())) // only alphabetic
                .collect::<Vec<String>>();
        } else {
            self.file_path = String::new();
            self.word_list = Vec::new();
        }
    }

    // predicates --------------------------------------------------------------

    pub fn iter_predicate_names(&self) -> impl Iterator<Item = &&str> {
        PREDICATE_NAMES.iter()
    }

    pub fn add_predicate(&mut self, index: usize) {
        if let Some(p) = WordPredicate::from_index(index) {
            self.predicates.push(p);
        }
    }

    pub fn get_predicate_string(&self, index: usize) -> String {
        // TODO: handle out of bounds
        self.predicates[index].get_string()
    }

    pub fn update_predicate(&mut self, index: usize, s: &str) {
        if let Some(p) = self.predicates.get_mut(index) {
            p.update(s);
        }
    }

    pub fn remove_predicate(&mut self, index: usize) {
        self.predicates.remove(index);
    }

    // word order --------------------------------------------------------------

    pub fn iter_order_names(&self) -> impl Iterator<Item = &&str> {
        ORDER_NAMES.iter()
    }

    pub fn set_order(&mut self, index: usize) {
        if let Some(o) = WordOrder::from_index(index) {
            self.word_order = o;
        }
    }

    pub fn sort(&mut self) {
        self.word_list.sort_by(|a, b| self.word_order.cmp(a, b));
    }

    // output

    pub fn iter_filtered(&self) -> impl Iterator<Item = &String> {
        self.word_list
            .iter()
            .filter(move |word| self.predicates.iter().all(|f| f.matches(word)))
    }
}
