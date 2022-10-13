// Logic

/// # Tautology
/// Representative of `true`
pub const T0: bool = true; 

/// # Fallacy
/// Representative of `false`
pub const F0: bool = false; 

/// Negation (Not)
/// # Example
/// ```rust
/// use formalsyslogic::week3::*;
/// assert_eq!(not(false,false), false);
/// assert_eq!(not(false,true), false);
/// assert_eq!(not(true,false), false);
/// assert_eq!(not(true,true), true);
/// ```
pub fn not(p: bool) -> bool {
    !p
}

/// Conjunction (AND)
/// # Example
/// ```rust
/// use formalsyslogic::week3::*;
/// assert_eq!(and(false,false), false);
/// assert_eq!(and(false,true), false);
/// assert_eq!(and(true,false), false);
/// assert_eq!(and(true,true), true);
/// ```
pub fn and(p: bool, q: bool) -> bool {
    p && q
}

/// Disjunction (Inclusive OR)
/// - p v q
/// # Example
/// ```rust
/// use formalsyslogic::week3::*;
/// assert_eq!(or(false,false), false);
/// assert_eq!(or(false,true), true);
/// assert_eq!(or(true,false), true);
/// assert_eq!(or(true,true), true);
/// ```
pub fn or(p: bool, q: bool) -> bool {
    p || q
}

/// # Exclusive OR 
/// - p -v- q
/// ### Example
/// ```rust
/// use formalsyslogic::week3::*;
/// assert_eq!(xor(false,false), false);
/// assert_eq!(xor(false,true), true);
/// assert_eq!(xor(true,false), true);
/// assert_eq!(xor(true,true), false);
/// ```
pub fn xor(p: bool, q: bool) -> bool {
    (p || q) && !(p == q)
}

/// # Conditional
/// - p -> q
/// ### Logical Equivalence
/// - (not p) or q
/// ### Example
/// ```rust
/// use formalsyslogic::week3::*;
/// assert_eq!(cond(false,false), true);
/// assert_eq!(cond(false,true), false);
/// assert_eq!(cond(true,false), false);
/// assert_eq!(cond(true,true), true);
/// 
/// assert_eq(
/// not(true)
/// true)
/// ```
pub fn cond(p: bool, q: bool) -> bool {
    or(not(p), q)
}

/// # Biconditional
/// - p <-> q
/// ### Logical equivalence
/// - (p -> q) v (q -> p)
/// ### Example
/// ```rust
/// use formalsyslogic::week3::*;
/// assert_eq!(cond(false,false), true);
/// assert_eq!(cond(false,true), false);
/// assert_eq!(cond(true,false), false);
/// assert_eq!(cond(true,true), true);
/// ```
pub fn bicond(p: bool, q: bool) -> bool {
    or(cond(p,q), cond(q,p))
}



