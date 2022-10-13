/// Factorial Function
/// # Examples
/// Finding the factorial of 4: 
/// ```
/// use formalsyslogic::week1::factorial;
/// assert_eq!(factorial(4), 24);
/// ```
/// 
pub fn factorial(x: u128) -> u128 {
    let mut tmp = 1;
    for i in 1..=x {
        tmp *= i;
    }
    tmp
}

/// Combination Function
/// # Examples:
/// Suppose that Helen is going to draw 5 cards from a standard deck of 52 cards.
/// In how many ways can her selection result in a hand with no clubs?
/// ```rust
/// use formalsyslogic::week1::*;
/// assert_eq!(C(39, 5), 575757)
/// ```
pub fn C(n: u128, r: u128) -> u128 {
    factorial(n)/(factorial(r) * factorial(n - r))
}

/// Permutation Function\
/// \
/// Example:\
/// How many different **permutations** can be obtained with the letters of the word COMPUTER?\
/// ```rust
/// use formalsyslogic::week1::*;
/// assert_eq!(P(8,8), 40320)
/// ```
/// Useful arithmatic knowledge is that of `P(n,0) = 1` and `P(n,n) = n!` 
pub fn P(n: u128, r: u128) -> u128 {
    factorial(n) / factorial(n-r)
}

/// Arrangements Function\
/// \
/// Example:\
/// How many different **arrangements** can be obtained with the letters of the word COMPUTER?\
/// ```rust
/// use formalsyslogic::week1::*;
/// assert_eq!(arrangements(8,8), 16777216); 
/// ```
pub fn arrangements(n: u128, r: u32) -> u128 {
    u128::pow(n, r)
}