use std::cmp::Ordering;

#[derive(Debug, Default)]
pub enum Sorter {
    #[default]
    Alphabetical,
    RevAlphabetical,
    Length,
    RevLength,
    Random,
}

impl Sorter {
    pub fn cmp(&self, left: &str, right: &str) -> Ordering {
        match self {
            Sorter::Alphabetical => left.cmp(right),
            Sorter::RevAlphabetical => left.cmp(right).reverse(),
            Sorter::Length => left.len().cmp(&right.len()),
            Sorter::RevLength => left.len().cmp(&right.len()).reverse(),
            Sorter::Random => todo!(),
        }
    }
}
