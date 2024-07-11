// const vec containing first 26 prime numbers
const PRIMES: [u64; 26] = [
    2,   // a
    3,   // b
    5,   // c
    7,   // d
    11,  // e
    13,  // f
    17,  // g
    19,  // h
    23,  // i
    29,  // j
    31,  // k
    37,  // l
    41,  // m
    43,  // n
    47,  // o
    53,  // p
    59,  // q
    61,  // r
    67,  // s
    71,  // t
    73,  // u
    79,  // v
    83,  // w
    89,  // x
    97,  // y
    101, // z
];

fn letter_to_index(letter: char) -> usize {
    letter as usize - 'a' as usize
}

fn prime_encoding(word: &str) -> u64 {
    word.chars()
        .map(|letter| PRIMES[letter_to_index(letter)])
        .product()
}
