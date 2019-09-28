use super::{LCons, LFoldOp, LFoldOpOutput, TList};
use crate::{
    boolean::Boolean,
    functional::{
        ApplyFunctor, BooleanAndFoldFunctor, BooleanOrFoldFunctor, Functor, MaxFoldFunctor,
        MinFoldFunctor, ProdFoldFunctor, SumFoldFunctor,
    },
};

// reduce max

/// A type operator that takes the maximum value among a [TList].
pub struct LReduceMaxFunctor {}

pub type LReduceMax<List> = ApplyFunctor<LReduceMaxFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceMaxFunctor
where
    Tail: TList + LFoldOp<Head, MaxFoldFunctor>,
{
    type Output = LFoldOpOutput<Tail, Head, MaxFoldFunctor>;
}

// reduce min

/// A type operator that takes the minimum value among a [TList].
pub struct LReduceMinFunctor {}

pub type LReduceMin<List> = ApplyFunctor<LReduceMinFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceMinFunctor
where
    Tail: TList + LFoldOp<Head, MinFoldFunctor>,
{
    type Output = LFoldOpOutput<Tail, Head, MinFoldFunctor>;
}

// reduce sum

/// A type operator that takes the summation of values in [TList].
pub struct LReduceSumFunctor {}

pub type LReduceSum<List> = ApplyFunctor<LReduceSumFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceSumFunctor
where
    Tail: TList + LFoldOp<Head, SumFoldFunctor>,
{
    type Output = LFoldOpOutput<Tail, Head, SumFoldFunctor>;
}

// reduce product

/// A type operator that takes the product of values in [TList].
pub struct LReduceProdFunctor {}

pub type LReduceProd<List> = ApplyFunctor<LReduceProdFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceProdFunctor
where
    Tail: TList + LFoldOp<Head, ProdFoldFunctor>,
{
    type Output = LFoldOpOutput<Tail, Head, ProdFoldFunctor>;
}

// reduce all

/// A type operator returns [True](crate::boolean::True) if all values in [TList] are [True](crate::boolean::True).
pub struct LReduceAllFunctor {}

pub type LReduceAll<List> = ApplyFunctor<LReduceAllFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceAllFunctor
where
    Tail: TList + LFoldOp<Head, BooleanAndFoldFunctor>,
    LFoldOpOutput<Tail, Head, BooleanAndFoldFunctor>: Boolean,
{
    type Output = LFoldOpOutput<Tail, Head, BooleanAndFoldFunctor>;
}

// reduce product

/// A type operator returns [True](crate::boolean::True) if any value in [TList] is [True](crate::boolean::True).
pub struct LReduceAnyFunctor {}

pub type LReduceAny<List> = ApplyFunctor<LReduceAnyFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceAnyFunctor
where
    Tail: TList + LFoldOp<Head, BooleanOrFoldFunctor>,
    LFoldOpOutput<Tail, Head, BooleanOrFoldFunctor>: Boolean,
{
    type Output = LFoldOpOutput<Tail, Head, BooleanOrFoldFunctor>;
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

    type Assert1 = IfEqualOutput<(), LReduceMax<List1>, U5>;
    type Assert2 = IfEqualOutput<(), LReduceMax<List2>, P7>;
    type Assert3 = IfEqualOutput<(), LReduceMax<List3>, P9>;

    type Assert4 = IfEqualOutput<(), LReduceMin<List1>, U0>;
    type Assert5 = IfEqualOutput<(), LReduceMin<List2>, N4>;
    type Assert6 = IfEqualOutput<(), LReduceMin<List3>, N8>;

    type Assert7 = IfEqualOutput<(), LReduceSum<List1>, U10>;
    type Assert8 = IfEqualOutput<(), LReduceSum<List2>, Z0>;
    type Assert9 = IfEqualOutput<(), LReduceSum<List3>, N4>;

    type Assert10 = IfEqualOutput<(), LReduceProd<List1>, U0>;
    type Assert11 = IfEqualOutput<(), LReduceProd<List2>, Z0>;
    type Assert12 = IfEqualOutput<(), LReduceProd<List3>, P1008>;

    type Assert13 = IfPredicateOutput<(), LReduceAll<TListType! {True, True}>>;
    type Assert14 = IfNotPredicateOutput<(), LReduceAll<TListType! {True, False, False}>>;

    type Assert15 = IfPredicateOutput<(), LReduceAny<TListType! {True, False, False}>>;
    type Assert16 = IfNotPredicateOutput<(), LReduceAll<TListType! {False, False}>>;

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
