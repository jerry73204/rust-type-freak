use super::{LCons, LFold, LFoldOutput, TList};
use crate::{
    boolean::Boolean,
    functional::{
        BooleanAndFoldFunc, BooleanOrFoldFunc, MaxFoldFunc, MinFoldFunc, ProdFoldFunc, SumFoldFunc,
    },
};

// reduce max

/// A type operator that takes the maximum value among a [TList].
pub trait LReduceMax
where
    Self: TList,
{
    type Output;
}

pub type LReduceMaxOutput<List> = <List as LReduceMax>::Output;

impl<Head, Tail> LReduceMax for LCons<Head, Tail>
where
    Tail: TList + LFold<Head, MaxFoldFunc>,
{
    type Output = LFoldOutput<Tail, Head, MaxFoldFunc>;
}

// reduce min

/// A type operator that takes the minimum value among a [TList].
pub trait LReduceMin
where
    Self: TList,
{
    type Output;
}

pub type LReduceMinOutput<List> = <List as LReduceMin>::Output;

impl<Head, Tail> LReduceMin for LCons<Head, Tail>
where
    Tail: TList + LFold<Head, MinFoldFunc>,
{
    type Output = LFoldOutput<Tail, Head, MinFoldFunc>;
}

// reduce sum

/// A type operator that takes the summation of values in [TList].
pub trait LReduceSum
where
    Self: TList,
{
    type Output;
}

pub type LReduceSumOutput<List> = <List as LReduceSum>::Output;

impl<Head, Tail> LReduceSum for LCons<Head, Tail>
where
    Tail: TList + LFold<Head, SumFoldFunc>,
{
    type Output = LFoldOutput<Tail, Head, SumFoldFunc>;
}

// reduce product

/// A type operator that takes the product of values in [TList].
pub trait LReduceProd
where
    Self: TList,
{
    type Output;
}

pub type LReduceProdOutput<List> = <List as LReduceProd>::Output;

impl<Head, Tail> LReduceProd for LCons<Head, Tail>
where
    Tail: TList + LFold<Head, ProdFoldFunc>,
{
    type Output = LFoldOutput<Tail, Head, ProdFoldFunc>;
}

// reduce all

/// A type operator returns [True](crate::boolean::True) if all values in [TList] are [True](crate::boolean::True).
pub trait LReduceAll
where
    Self: TList,
    Self::Output: Boolean,
{
    type Output;
}

pub type LReduceAllOutput<List> = <List as LReduceAll>::Output;

impl<Head, Tail> LReduceAll for LCons<Head, Tail>
where
    Tail: TList + LFold<Head, BooleanAndFoldFunc>,
    LFoldOutput<Tail, Head, BooleanAndFoldFunc>: Boolean,
{
    type Output = LFoldOutput<Tail, Head, BooleanAndFoldFunc>;
}

// reduce product

/// A type operator returns [True](crate::boolean::True) if any value in [TList] is [True](crate::boolean::True).
pub trait LReduceAny
where
    Self: TList,
{
    type Output;
}

pub type LReduceAnyOutput<List> = <List as LReduceAny>::Output;

impl<Head, Tail> LReduceAny for LCons<Head, Tail>
where
    Tail: TList + LFold<Head, BooleanOrFoldFunc>,
    LFoldOutput<Tail, Head, BooleanOrFoldFunc>: Boolean,
{
    type Output = LFoldOutput<Tail, Head, BooleanOrFoldFunc>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        control::{IfEqualOutput, IfNotPredicateOutput, IfPredicateOutput},
        TListType,
    };
    use typenum::consts::*;

    type List1 = TListType! {U2, U5, U3, U0};
    type List2 = TListType! {N3, P7, Z0, N4};
    type List3 = TListType! {N7, P9, P2, N8};

    type Assert1 = IfEqualOutput<(), LReduceMaxOutput<List1>, U5>;
    type Assert2 = IfEqualOutput<(), LReduceMaxOutput<List2>, P7>;
    type Assert3 = IfEqualOutput<(), LReduceMaxOutput<List3>, P9>;

    type Assert4 = IfEqualOutput<(), LReduceMinOutput<List1>, U0>;
    type Assert5 = IfEqualOutput<(), LReduceMinOutput<List2>, N4>;
    type Assert6 = IfEqualOutput<(), LReduceMinOutput<List3>, N8>;

    type Assert7 = IfEqualOutput<(), LReduceSumOutput<List1>, U10>;
    type Assert8 = IfEqualOutput<(), LReduceSumOutput<List2>, Z0>;
    type Assert9 = IfEqualOutput<(), LReduceSumOutput<List3>, N4>;

    type Assert10 = IfEqualOutput<(), LReduceProdOutput<List1>, U0>;
    type Assert11 = IfEqualOutput<(), LReduceProdOutput<List2>, Z0>;
    type Assert12 = IfEqualOutput<(), LReduceProdOutput<List3>, P1008>;

    type Assert13 = IfPredicateOutput<(), LReduceAllOutput<TListType! {True, True}>>;
    type Assert14 = IfNotPredicateOutput<(), LReduceAllOutput<TListType! {True, False, False}>>;

    type Assert15 = IfPredicateOutput<(), LReduceAnyOutput<TListType! {True, False, False}>>;
    type Assert16 = IfNotPredicateOutput<(), LReduceAllOutput<TListType! {False, False}>>;

    #[test]
    fn tlist_reduction_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
        let _: Assert9 = ();
        let _: Assert10 = ();
        let _: Assert11 = ();
        let _: Assert12 = ();
        let _: Assert13 = ();
        let _: Assert14 = ();
        let _: Assert15 = ();
        let _: Assert16 = ();
    }
}
