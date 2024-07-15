#[derive(Debug)]
pub enum Predicate {
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

pub const PREDICATES: [&str; 6] = [
    "Length",
    "Starts with",
    "Ends with",
    "Contains",
    "Using letters",
    "Scrabble playable",
];

impl Predicate {

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Predicate::Length(0)),
            1 => Some(Predicate::StartsWith(String::new())),
            2 => Some(Predicate::EndsWith(String::new())),
            3 => Some(Predicate::Contains(String::new())),
            4 => Some(Predicate::UsingLetters(String::new())),
            5 => Some(Predicate::ScrabblePlayable(String::new())),
            _ => None,
        }
    }

    pub fn matches(&self, word: &str) -> bool {
        match self {
            Predicate::Length(len) => word.len() == *len,
            Predicate::StartsWith(prefix) => word.starts_with(prefix),
            Predicate::EndsWith(suffix) => word.ends_with(suffix),
            Predicate::Contains(substring) => word.contains(substring),
            Predicate::UsingLetters(letters) => {
                for l in word.chars() {
                    if !letters.contains(l) {
                        return false;
                    }
                }
                true
            }
            Predicate::ScrabblePlayable(tiles) => {
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
            Predicate::Length(len) => len.to_string(),
            Predicate::StartsWith(prefix) => prefix.to_string(),
            Predicate::EndsWith(suffix) => suffix.to_string(),
            Predicate::Contains(substring) => substring.to_string(),
            Predicate::UsingLetters(letters) => letters.to_string(),
            Predicate::ScrabblePlayable(tiles) => tiles.to_string(),
        }
    }

    pub fn update(&mut self, s: &str) {
        match self {
            Predicate::Length(len) => {
                if let Ok(len2) = s.parse() {
                    *len = len2;
                } else {
                    *len = 0;
                }
            }
            Predicate::StartsWith(prefix) => {
                *prefix = s.to_string();
            }
            Predicate::EndsWith(suffix) => {
                *suffix = s.to_string();
            }
            Predicate::Contains(substring) => {
                *substring = s.to_string();
            }
            Predicate::UsingLetters(letters) => {
                *letters = s.to_string();
            }
            Predicate::ScrabblePlayable(tiles) => {
                *tiles = s.to_string();
            }
        }
    }
}

impl std::fmt::Display for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Predicate::Length(len) => write!(f, "Length: {}", len),
            Predicate::StartsWith(prefix) => write!(f, "Starts with: {}", prefix),
            Predicate::EndsWith(suffix) => write!(f, "Ends with: {}", suffix),
            Predicate::Contains(substring) => write!(f, "Contains: {}", substring),
            Predicate::UsingLetters(letters) => write!(f, "Using letters: {}", letters),
            Predicate::ScrabblePlayable(tiles) => write!(f, "Scrabble playable: {}", tiles),
        }
    }
}
