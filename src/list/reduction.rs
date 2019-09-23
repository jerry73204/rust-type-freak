use super::{LCons, LNil, TList};
use std::ops::{Add, Mul};
use typenum::{Max, Maximum, Min, Minimum, Prod, Sum};

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
    Tail: TList + LReducingMax<Head>,
{
    type Output = LReducingMaxOutput<Tail, Head>;
}

/// A type operator that takes the maximum value among an argument and a [TList].
///
/// It is an auxiliary trait for [LReduceMax].
pub trait LReducingMax<Prev>
where
    Self: TList,
{
    type Output;
}

pub type LReducingMaxOutput<List, Prev> = <List as LReducingMax<Prev>>::Output;

impl<Prev> LReducingMax<Prev> for LNil {
    type Output = Prev;
}

impl<Prev, Head, Tail> LReducingMax<Prev> for LCons<Head, Tail>
where
    Prev: Max<Head>,
    Tail: TList + LReducingMax<Maximum<Prev, Head>>,
{
    type Output = LReducingMaxOutput<Tail, Maximum<Prev, Head>>;
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
    Tail: TList + LReducingMin<Head>,
{
    type Output = LReducingMinOutput<Tail, Head>;
}

/// A type operator that takes the minimum value among an argument and a [TList].
///
/// It is an auxiliary trait for [LReduceMin].
pub trait LReducingMin<Prev>
where
    Self: TList,
{
    type Output;
}

pub type LReducingMinOutput<List, Prev> = <List as LReducingMin<Prev>>::Output;

impl<Prev> LReducingMin<Prev> for LNil {
    type Output = Prev;
}

impl<Prev, Head, Tail> LReducingMin<Prev> for LCons<Head, Tail>
where
    Prev: Min<Head>,
    Tail: TList + LReducingMin<Minimum<Prev, Head>>,
{
    type Output = LReducingMinOutput<Tail, Minimum<Prev, Head>>;
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
    Tail: TList + LReducingSum<Head>,
{
    type Output = LReducingSumOutput<Tail, Head>;
}

/// A type operator that takes the summation among an argument and values in [TList].
///
/// It is an auxiliary trait for [LReduceSum].
pub trait LReducingSum<Prev>
where
    Self: TList,
{
    type Output;
}

pub type LReducingSumOutput<List, Prev> = <List as LReducingSum<Prev>>::Output;

impl<Prev> LReducingSum<Prev> for LNil {
    type Output = Prev;
}

impl<Prev, Head, Tail> LReducingSum<Prev> for LCons<Head, Tail>
where
    Prev: Add<Head>,
    Tail: TList + LReducingSum<Sum<Prev, Head>>,
{
    type Output = LReducingSumOutput<Tail, Sum<Prev, Head>>;
}

// reduce product

/// A type operator that takes the product of values in [TList].
pub trait LReduceProduct
where
    Self: TList,
{
    type Output;
}

pub type LReduceProductOutput<List> = <List as LReduceProduct>::Output;

impl<Head, Tail> LReduceProduct for LCons<Head, Tail>
where
    Tail: TList + LReducingProduct<Head>,
{
    type Output = LReducingProductOutput<Tail, Head>;
}

/// A type operator that takes the product value among an argument and values [TList].
///
/// It is an auxiliary trait for [LReduceProduct].
pub trait LReducingProduct<Prev>
where
    Self: TList,
{
    type Output;
}

pub type LReducingProductOutput<List, Prev> = <List as LReducingProduct<Prev>>::Output;

impl<Prev> LReducingProduct<Prev> for LNil {
    type Output = Prev;
}

impl<Prev, Head, Tail> LReducingProduct<Prev> for LCons<Head, Tail>
where
    Prev: Mul<Head>,
    Tail: TList + LReducingProduct<Prod<Prev, Head>>,
{
    type Output = LReducingProductOutput<Tail, Prod<Prev, Head>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfEqualOutput, TListType};
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

    type Assert10 = IfEqualOutput<(), LReduceProductOutput<List1>, U0>;
    type Assert11 = IfEqualOutput<(), LReduceProductOutput<List2>, Z0>;
    type Assert12 = IfEqualOutput<(), LReduceProductOutput<List3>, P1008>;

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
    }
}
