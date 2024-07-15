#[derive(Debug)]
pub enum WordFilter {
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

impl WordFilter {
    pub fn matches(&self, word: &String) -> bool {
        match self {
            WordFilter::Length(len) => word.len() == *len,
            WordFilter::StartsWith(prefix) => word.starts_with(prefix),
            WordFilter::EndsWith(suffix) => word.ends_with(suffix),
            WordFilter::Contains(substring) => word.contains(substring),
            WordFilter::UsingLetters(letters) => {
                for l in word.chars() {
                    if !letters.contains(l) {
                        return false;
                    }
                }
                true
            }
            WordFilter::ScrabblePlayable(tiles) => {
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
            WordFilter::Length(len) => len.to_string(),
            WordFilter::StartsWith(prefix) => prefix.to_string(),
            WordFilter::EndsWith(suffix) => suffix.to_string(),
            WordFilter::Contains(substring) => substring.to_string(),
            WordFilter::UsingLetters(letters) => letters.to_string(),
            WordFilter::ScrabblePlayable(tiles) => tiles.to_string(),
        }
    }

    pub fn update(&mut self, s: &str) {
        match self {
            WordFilter::Length(len) => {
                if let Ok(len2) = s.parse() {
                    *len = len2;
                } else {
                    *len = 0;
                }
            }
            WordFilter::StartsWith(prefix) => {
                *prefix = s.to_string();
            }
            WordFilter::EndsWith(suffix) => {
                *suffix = s.to_string();
            }
            WordFilter::Contains(substring) => {
                *substring = s.to_string();
            }
            WordFilter::UsingLetters(letters) => {
                *letters = s.to_string();
            }
            WordFilter::ScrabblePlayable(tiles) => {
                *tiles = s.to_string();
            }
        }
    }
}

impl std::fmt::Display for WordFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WordFilter::Length(len) => write!(f, "Length: {}", len),
            WordFilter::StartsWith(prefix) => write!(f, "Starts with: {}", prefix),
            WordFilter::EndsWith(suffix) => write!(f, "Ends with: {}", suffix),
            WordFilter::Contains(substring) => write!(f, "Contains: {}", substring),
            WordFilter::UsingLetters(letters) => write!(f, "Using letters: {}", letters),
            WordFilter::ScrabblePlayable(tiles) => write!(f, "Scrabble playable: {}", tiles),
        }
    }
}
