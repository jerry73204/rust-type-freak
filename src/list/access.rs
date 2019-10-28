use super::{marker::NonEmptyTList, LCons, LNil, LReverse, LReverseMap, TList};
use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyMap, Map},
};
use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};
use typenum::{Add1, Bit, NonZero, Sub1, UInt, Unsigned, B1, U0};

// index of item

/// A type operator that returns the position of `Target` type in [TList].
///
/// The returned outcome always implements [Unsigned](typenum::Unsigned)
/// trait. The `Index` argument can be left unspecified.
pub trait LIndexOfOp<Target, Index>
where
    Self: TList,
    Index: Counter,
    Self::Output: Unsigned,
{
    type Output;
}

pub type LIndexOfOpOutput<List, Target, Index> = <List as LIndexOfOp<Target, Index>>::Output;

impl<Target, Tail> LIndexOfOp<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = U0;
}

impl<Target, Index, NonTarget, Tail> LIndexOfOp<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LIndexOfOp<Target, Index>,
    LIndexOfOpOutput<Tail, Target, Index>: Add<B1>,
    Add1<LIndexOfOpOutput<Tail, Target, Index>>: Unsigned,
{
    type Output = Add1<LIndexOfOpOutput<Tail, Target, Index>>;
}

/// A [Map] that returns the index of `Target` in [TList].
pub struct LIndexOfMap<Target, Index>
where
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type LIndexOf<List, Target, Index> = ApplyMap<LIndexOfMap<Target, Index>, List>;

impl<List, Target, Index> Map<List> for LIndexOfMap<Target, Index>
where
    List: TList + LIndexOfOp<Target, Index>,
    Index: Counter,
{
    type Output = LIndexOfOpOutput<List, Target, Index>;
}

// index of many

/// Gets indexes of multiple types from [TList].
///
/// The `Targets` argument is a [TList] of queried types.
/// The `Indexes` can be left unspecified.
pub trait LIndexOfManyOp<Targets, Indexes>
where
    Self: TList,
    Targets: TList,
    Indexes: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LIndexOfManyOpOutput<List, Targets, Indexes> =
    <List as LIndexOfManyOp<Targets, Indexes>>::Output;

impl<List> LIndexOfManyOp<LNil, LNil> for List
where
    List: TList,
{
    type Output = LNil;
}

impl<List, Index, IRemain, Target, TRemain>
    LIndexOfManyOp<LCons<Target, TRemain>, LCons<Index, IRemain>> for List
where
    List: NonEmptyTList,
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Self: LIndexOfManyOp<TRemain, IRemain> + LIndexOfOp<Target, Index>,
{
    type Output =
        LCons<LIndexOfOpOutput<Self, Target, Index>, LIndexOfManyOpOutput<Self, TRemain, IRemain>>;
}

/// A [Map] that returns indexes of multiple `Targets`.
pub struct LIndexOfManyMap<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(Targets, Indexes)>,
}

pub type LIndexOfMany<List, Targets, Indexes> = ApplyMap<LIndexOfManyMap<Targets, Indexes>, List>;

impl<List, Targets, Indexes> Map<List> for LIndexOfManyMap<Targets, Indexes>
where
    List: TList + LIndexOfManyOp<Targets, Indexes>,
    Targets: TList,
    Indexes: TList,
{
    type Output = LIndexOfManyOpOutput<List, Targets, Indexes>;
}

// get by position

/// A [Map] that gets element at `Position` in input [TList].
pub struct LGetByPositionMap<Position>
where
    Position: Unsigned,
{
    _phantom: PhantomData<Position>,
}

pub type LGetByPosition<List, Position> = ApplyMap<LGetByPositionMap<Position>, List>;

impl<Head, Tail> Map<LCons<Head, Tail>> for LGetByPositionMap<U0>
where
    Tail: TList,
{
    type Output = Head;
}

impl<Head, Tail, U, B> Map<LCons<Head, Tail>> for LGetByPositionMap<UInt<U, B>>
where
    Tail: TList,
    U: Unsigned,
    B: Bit,
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: Unsigned,
    LGetByPositionMap<Sub1<UInt<U, B>>>: Map<Tail>,
{
    type Output = LGetByPosition<Tail, Sub1<UInt<U, B>>>;
}

// get by backward position

/// A [Map] that gets element at `Position` in input [TList].
pub struct LGetByBackwardPositionMap<Position>
where
    Position: Unsigned + NonZero,
{
    _phantom: PhantomData<Position>,
}

pub type LGetByBackwardPosition<List, Position> =
    ApplyMap<LGetByBackwardPositionMap<Position>, List>;

impl<List, Position> Map<List> for LGetByBackwardPositionMap<Position>
where
    List: TList,
    Position: Unsigned + NonZero + Sub<B1>,
    Sub1<Position>: Unsigned,
    LReverseMap: Map<List>,
    LGetByPositionMap<Sub1<Position>>: Map<LReverse<List>>,
{
    type Output = LGetByPosition<LReverse<List>, Sub1<Position>>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        control::{IfOutput, IfSameOutput},
        TListType,
    };
    use typenum::consts::*;

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;

    type SomeList = TListType![A, B, C];

    // index of tiem
    type Assert1<Idx> = AssertSame<LIndexOf<SomeList, A, Idx>, U0>;
    type Assert2<Idx> = AssertSame<LIndexOf<SomeList, B, Idx>, U1>;
    type Assert3<Idx> = AssertSame<LIndexOf<SomeList, C, Idx>, U2>;

    // index of multiple items
    type Indexes<Idx> = LIndexOfMany<SomeList, TListType![C, A, B], Idx>;
    type Assert4<Idx> = AssertSame<Indexes<Idx>, TListType![U2, U0, U1]>;

    // get by position
    type Assert5 = IfOutput<
        (),
        (
            AssertSame<LGetByPosition<SomeList, U0>, A>,
            AssertSame<LGetByPosition<SomeList, U1>, B>,
            AssertSame<LGetByPosition<SomeList, U2>, C>,
        ),
    >;

    // get by backward position
    type Assert6 = IfOutput<
        (),
        (
            AssertSame<LGetByBackwardPosition<SomeList, U1>, C>,
            AssertSame<LGetByBackwardPosition<SomeList, U2>, B>,
            AssertSame<LGetByBackwardPosition<SomeList, U3>, A>,
        ),
    >;

    #[test]
    fn tlist_access_test() {
        let _: Assert1<_> = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_> = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
    }
}
