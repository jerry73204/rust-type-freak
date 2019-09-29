use super::{marker::NonEmptyTList, LCons, LNil, TList};
use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
};
use std::{marker::PhantomData, ops::Add};
use typenum::{Add1, Unsigned, B1, U0};

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

/// A [Functor] that returns the index of `Target` in [TList].
pub struct LIndexOfFunctor<Target, Index>
where
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type LIndexOf<List, Target, Index> = ApplyFunctor<LIndexOfFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for LIndexOfFunctor<Target, Index>
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

/// A [Functor] that returns indexes of multiple `Targets`.
pub struct LIndexOfManyFunctor<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(Targets, Indexes)>,
}

pub type LIndexOfMany<List, Targets, Indexes> =
    ApplyFunctor<LIndexOfManyFunctor<Targets, Indexes>, List>;

impl<List, Targets, Indexes> Functor<List> for LIndexOfManyFunctor<Targets, Indexes>
where
    List: TList + LIndexOfManyOp<Targets, Indexes>,
    Targets: TList,
    Indexes: TList,
{
    type Output = LIndexOfManyOpOutput<List, Targets, Indexes>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, TListType};
    use typenum::consts::*;

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;

    type SomeList = TListType![A, B, C];

    // index of tiem
    type Assert13<Idx> = AssertSame<LIndexOf<SomeList, A, Idx>, U0>;
    type Assert14<Idx> = AssertSame<LIndexOf<SomeList, B, Idx>, U1>;
    type Assert15<Idx> = AssertSame<LIndexOf<SomeList, C, Idx>, U2>;

    // index of multiple items
    type Indexes<Idx> = LIndexOfMany<SomeList, TListType![C, A, B], Idx>;
    type Assert16<Idx> = AssertSame<Indexes<Idx>, TListType![U2, U0, U1]>;

    #[test]
    fn tlist_test() {
        let _: Assert13<_> = ();
        let _: Assert14<_> = ();
        let _: Assert15<_> = ();
        let _: Assert16<_> = ();
    }
}
