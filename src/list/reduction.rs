use super::{LCons, LFoldOp, LFoldOpOutput, TList};
use crate::{
    boolean::{AndComposePredicate, Boolean, OrComposePredicate},
    functional::{ApplyMap, Map},
    numeric::{MaxComposeMap, MinComposeMap, ProdComposeMap, SumComposeMap},
};

// reduce max

/// A [Map] that takes the maximum value among a [TList].
pub struct LReduceMaxComposeMap {}

pub type LReduceMax<List> = ApplyMap<LReduceMaxComposeMap, List>;

impl<Head, Tail> Map<LCons<Head, Tail>> for LReduceMaxComposeMap
where
    Tail: TList + LFoldOp<Head, MaxComposeMap>,
{
    type Output = LFoldOpOutput<Tail, Head, MaxComposeMap>;
}

// reduce min

/// A [Map] that takes the minimum value among a [TList].
pub struct LReduceMinComposeMap {}

pub type LReduceMin<List> = ApplyMap<LReduceMinComposeMap, List>;

impl<Head, Tail> Map<LCons<Head, Tail>> for LReduceMinComposeMap
where
    Tail: TList + LFoldOp<Head, MinComposeMap>,
{
    type Output = LFoldOpOutput<Tail, Head, MinComposeMap>;
}

// reduce sum

/// A [Map] that takes the summation of values in [TList].
pub struct LReduceSumComposeMap {}

pub type LReduceSum<List> = ApplyMap<LReduceSumComposeMap, List>;

impl<Head, Tail> Map<LCons<Head, Tail>> for LReduceSumComposeMap
where
    Tail: TList + LFoldOp<Head, SumComposeMap>,
{
    type Output = LFoldOpOutput<Tail, Head, SumComposeMap>;
}

// reduce product

/// A [Map] that takes the product of values in [TList].
pub struct LReduceProdComposeMap {}

pub type LReduceProd<List> = ApplyMap<LReduceProdComposeMap, List>;

impl<Head, Tail> Map<LCons<Head, Tail>> for LReduceProdComposeMap
where
    Tail: TList + LFoldOp<Head, ProdComposeMap>,
{
    type Output = LFoldOpOutput<Tail, Head, ProdComposeMap>;
}

// reduce all

/// A [Map] that returns [True](crate::boolean::True) if all values in [TList] are [True](crate::boolean::True).
pub struct LReduceAllMap {}

pub type LReduceAll<List> = ApplyMap<LReduceAllMap, List>;

impl<Head, Tail> Map<LCons<Head, Tail>> for LReduceAllMap
where
    Tail: TList + LFoldOp<Head, AndComposePredicate>,
    LFoldOpOutput<Tail, Head, AndComposePredicate>: Boolean,
{
    type Output = LFoldOpOutput<Tail, Head, AndComposePredicate>;
}

// reduce product

/// A [Map] that returns [True](crate::boolean::True) if any value in [TList] is [True](crate::boolean::True).
pub struct LReduceAnyMap {}

pub type LReduceAny<List> = ApplyMap<LReduceAnyMap, List>;

impl<Head, Tail> Map<LCons<Head, Tail>> for LReduceAnyMap
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
