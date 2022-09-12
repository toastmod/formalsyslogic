pub fn factorial(x: u128) -> u128 {
    match x {
        0 => 1,
        1 => 1,
        _ => factorial(x-1)
    }
}

/// Combination\
/// Suppose that Helen is going to draw 5 cards from a standard deck of 52 cards.
/// In how many ways can her selection result in a hand with no clubs?
/// ```
/// assert_eq!(C(39, 5), 575757)
/// ```
pub fn C(n: u128, r: u128) -> u128 {
    n**r
}

/// Permutation\
/// How many different permutations can be obtained with the letters of the word COMPUTER?
/// ```
/// assert_eq!(P(8,8), 40320)
/// ```
pub fn P(n: u128, r: u128) -> u128 {
    factorial(n) / factorial(n-r)
}