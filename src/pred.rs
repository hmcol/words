#[derive(Debug)]
pub enum WordPredicate {
    Length(usize),
    StartsWith(String),
    EndsWith(String),
    Contains(String),
    /// Find words that are spelled using only the given letters, repeat letters allowed
    ///
    /// given `letters` a "set" of letters, we only require that each letter of the word is in `letters`
    UsingLetters(String),
    /// Find words that could be played given a set of tiles in a scrabble game
    ///
    /// `tiles` must be a string containing only letters and possibly question marks to represent blank tiles
    ScrabblePlayable(String),
}

pub const PREDICATE_NAMES: [&str; 6] = [
    "Length",
    "Starts with",
    "Ends with",
    "Contains",
    "Using letters",
    "Scrabble playable",
];

impl WordPredicate {

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(WordPredicate::Length(0)),
            1 => Some(WordPredicate::StartsWith(String::new())),
            2 => Some(WordPredicate::EndsWith(String::new())),
            3 => Some(WordPredicate::Contains(String::new())),
            4 => Some(WordPredicate::UsingLetters(String::new())),
            5 => Some(WordPredicate::ScrabblePlayable(String::new())),
            _ => None,
        }
    }

    pub fn matches(&self, word: &str) -> bool {
        match self {
            WordPredicate::Length(len) => word.len() == *len,
            WordPredicate::StartsWith(prefix) => word.starts_with(prefix),
            WordPredicate::EndsWith(suffix) => word.ends_with(suffix),
            WordPredicate::Contains(substring) => word.contains(substring),
            WordPredicate::UsingLetters(letters) => {
                for l in word.chars() {
                    if !letters.contains(l) {
                        return false;
                    }
                }
                true
            }
            WordPredicate::ScrabblePlayable(tiles) => {
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
            }
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            WordPredicate::Length(len) => len.to_string(),
            WordPredicate::StartsWith(prefix) => prefix.to_string(),
            WordPredicate::EndsWith(suffix) => suffix.to_string(),
            WordPredicate::Contains(substring) => substring.to_string(),
            WordPredicate::UsingLetters(letters) => letters.to_string(),
            WordPredicate::ScrabblePlayable(tiles) => tiles.to_string(),
        }
    }

    pub fn update(&mut self, s: &str) {
        match self {
            WordPredicate::Length(len) => {
                if let Ok(len2) = s.parse() {
                    *len = len2;
                } else {
                    *len = 0;
                }
            }
            WordPredicate::StartsWith(prefix) => {
                *prefix = s.to_string();
            }
            WordPredicate::EndsWith(suffix) => {
                *suffix = s.to_string();
            }
            WordPredicate::Contains(substring) => {
                *substring = s.to_string();
            }
            WordPredicate::UsingLetters(letters) => {
                *letters = s.to_string();
            }
            WordPredicate::ScrabblePlayable(tiles) => {
                *tiles = s.to_string();
            }
        }
    }
}

impl std::fmt::Display for WordPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WordPredicate::Length(len) => write!(f, "Length: {}", len),
            WordPredicate::StartsWith(prefix) => write!(f, "Starts with: {}", prefix),
            WordPredicate::EndsWith(suffix) => write!(f, "Ends with: {}", suffix),
            WordPredicate::Contains(substring) => write!(f, "Contains: {}", substring),
            WordPredicate::UsingLetters(letters) => write!(f, "Using letters: {}", letters),
            WordPredicate::ScrabblePlayable(tiles) => write!(f, "Scrabble playable: {}", tiles),
        }
    }
}
