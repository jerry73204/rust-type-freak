use super::{LCons, LFoldOp, LFoldOpOutput, TList};
use crate::{
    boolean::{AndComposePredicate, Boolean, OrComposePredicate},
    functional::{ApplyFunctor, Functor},
    numeric::{MaxComposeFunctor, MinComposeFunctor, ProdComposeFunctor, SumComposeFunctor},
};

// reduce max

/// A [Functor] that takes the maximum value among a [TList].
pub struct LReduceMaxComposeFunctor {}

pub type LReduceMax<List> = ApplyFunctor<LReduceMaxComposeFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceMaxComposeFunctor
where
    Tail: TList + LFoldOp<Head, MaxComposeFunctor>,
{
    type Output = LFoldOpOutput<Tail, Head, MaxComposeFunctor>;
}

// reduce min

/// A [Functor] that takes the minimum value among a [TList].
pub struct LReduceMinComposeFunctor {}

pub type LReduceMin<List> = ApplyFunctor<LReduceMinComposeFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceMinComposeFunctor
where
    Tail: TList + LFoldOp<Head, MinComposeFunctor>,
{
    type Output = LFoldOpOutput<Tail, Head, MinComposeFunctor>;
}

// reduce sum

/// A [Functor] that takes the summation of values in [TList].
pub struct LReduceSumComposeFunctor {}

pub type LReduceSum<List> = ApplyFunctor<LReduceSumComposeFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceSumComposeFunctor
where
    Tail: TList + LFoldOp<Head, SumComposeFunctor>,
{
    type Output = LFoldOpOutput<Tail, Head, SumComposeFunctor>;
}

// reduce product

/// A [Functor] that takes the product of values in [TList].
pub struct LReduceProdComposeFunctor {}

pub type LReduceProd<List> = ApplyFunctor<LReduceProdComposeFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceProdComposeFunctor
where
    Tail: TList + LFoldOp<Head, ProdComposeFunctor>,
{
    type Output = LFoldOpOutput<Tail, Head, ProdComposeFunctor>;
}

// reduce all

/// A [Functor] that returns [True](crate::boolean::True) if all values in [TList] are [True](crate::boolean::True).
pub struct LReduceAllFunctor {}

pub type LReduceAll<List> = ApplyFunctor<LReduceAllFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceAllFunctor
where
    Tail: TList + LFoldOp<Head, AndComposePredicate>,
    LFoldOpOutput<Tail, Head, AndComposePredicate>: Boolean,
{
    type Output = LFoldOpOutput<Tail, Head, AndComposePredicate>;
}

// reduce product

/// A [Functor] that returns [True](crate::boolean::True) if any value in [TList] is [True](crate::boolean::True).
pub struct LReduceAnyFunctor {}

pub type LReduceAny<List> = ApplyFunctor<LReduceAnyFunctor, List>;

impl<Head, Tail> Functor<LCons<Head, Tail>> for LReduceAnyFunctor
where
    Tail: TList + LFoldOp<Head, OrComposePredicate>,
    LFoldOpOutput<Tail, Head, OrComposePredicate>: Boolean,
{
    type Output = LFoldOpOutput<Tail, Head, OrComposePredicate>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        control::{IfEqualOutput, IfNotPredicateOutput, IfPredicateOutput},
        TListType,
    };
    use typenum::consts::*;

    type List1 = TListType![U2, U5, U3, U0];
    type List2 = TListType![N3, P7, Z0, N4];
    type List3 = TListType![N7, P9, P2, N8];

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

    type Assert13 = IfPredicateOutput<(), LReduceAll<TListType![True, True]>>;
    type Assert14 = IfNotPredicateOutput<(), LReduceAll<TListType![True, False, False]>>;

    type Assert15 = IfPredicateOutput<(), LReduceAny<TListType![True, False, False]>>;
    type Assert16 = IfNotPredicateOutput<(), LReduceAll<TListType![False, False]>>;

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
