use std::cmp::Ordering;

#[derive(Debug, Default)]
pub enum WordOrder {
    #[default]
    Alphabetical,
    RevAlphabetical,
    Length,
    RevLength,
    Random,
}

pub const ORDER_NAMES: [&str; 5] = [
    "Alphabetical",
    "RevAlphabetical",
    "Length",
    "RevLength",
    "Random",
];
 
impl WordOrder {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(WordOrder::Alphabetical),
            1 => Some(WordOrder::RevAlphabetical),
            2 => Some(WordOrder::Length),
            3 => Some(WordOrder::RevLength),
            4 => Some(WordOrder::Random),
            _ => None,
        }
    }

    pub fn cmp(&self, left: &str, right: &str) -> Ordering {
        match self {
            WordOrder::Alphabetical => left.cmp(right),
            WordOrder::RevAlphabetical => left.cmp(right).reverse(),
            WordOrder::Length => left.len().cmp(&right.len()),
            WordOrder::RevLength => left.len().cmp(&right.len()).reverse(),
            WordOrder::Random => todo!(),
        }
    }
}