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

    pub fn to_string(&self) -> String {
        match self {
            WordFilter::Length(len) => format!("Length: {}", len),
            WordFilter::StartsWith(prefix) => format!("Starts with: {}", prefix),
            WordFilter::EndsWith(suffix) => format!("Ends with: {}", suffix),
            WordFilter::Contains(substring) => format!("Contains: {}", substring),
            WordFilter::UsingLetters(letters) => format!("Using letters: {}", letters),
            WordFilter::ScrabblePlayable(tiles) => format!("Scrabble playable: {}", tiles),
        }
    }
}
