/// # Mathematical Induction
/// Assume an arbitrarily chosen `k` value...
/// come back to this ^^^
///
/// ### Example 1
/// p1: Every ambassador speaks only to diplomats\
/// AND p2: some ambassador speaks to someone.
/// Therefore there is a diplomat.
/// ```
/// A(x): "x is an ambassador"
/// D(x): "x is a diplomat"
/// C(x,y): "x speaks with y"
/// 
/// Goal: EQ_x D(x)
/// 
/// 1: UQ_x UQ_y [( C(x,y) AND D(y)) -> D(y)]   (PREMISE)
/// 2: EQ_x EQ_y [A(x) AND C(x,y)]  (PREMISE)
/// 3: EQ_y [A(r) AND C(r,y)]   ...2 ExesSpec, r is specific;
/// 4: A(r) AND C(r,s)  ...3 ExesSpec, s is specific;
/// 5: C(r,s) AND A(r)  ...4 Commutative
/// 6: UQ_y [(C(r,y) AND A(r)) -> D(y)] ...1 UnivSpec, r is specific (still)
/// 7: (C(r,s) AND A(r)) -> D(s) ...6 UnivSpec, s is specific (still)
/// ```
/// 
/// ### Example 2
/// 1: ((a OR b) AND c) -> d
/// 2: d -> (b -> e)
/// 3: b AND (e OR g)
/// 
/// ### Example 3
/// 
/// Goal: m -> (e AND h)
/// 
/// 1: b -> (h AND e)   (PREMISE)
/// 2: m -> b   (PREMISE)
/// 3: m    (assume for hyp. reason)
/// 4: b    2,3 modus ponens
/// 5: h AND e  4,1 modus ponens
/// 6: e AND h  5, commutative
/// 7: m -> (e AND h)   3-6, hyp. reasoning 



