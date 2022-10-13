use super::simple::*;

fn test_pq(f: fn(bool,bool)->()) {
    f(true,false);
    f(true,true);
    f(false,false);
    f(false,true);
}

/// # DeMorgan's Law
/// The negation of the conjunction (or) of two statements results in the disjunction (and) of their negations.\
/// Therefore the following are equivalent.\ 
/// - not (p v q)
/// - not(p) and not(q)
/// Or vice versa:
/// - not(p and q)
/// - not(p) v not(q)
/// ```
/// use formalsyslogic::week3::laws::*;
/// test_pq(demorgan);
/// ```
fn demorgan(p: bool, q: bool) {
    assert_eq!(
        not(or(p,q)),
        and(not(p), not(q))
    );
    assert_eq!(
        not(and(p,q)),
        or(not(p), not(q))
    );
}

/// # Law of Double Negation
/// The negation of a negation is equivalent to itself
fn double_neg(p: bool) {
    assert_eq!(not(not(p)), p)
}

/// # Distributive Laws
fn distributive() {

}

