use super::{LCons, LNil, TList};
use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
};
use std::marker::PhantomData;

// remove

/// Removes `Target` type from [TList].
///
/// The auxiliary `Index` argument is intended for
/// list traversal. It can be left unspecified and
/// the compiler will figure it out.
pub trait LRemoveAtOp<Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Target, Tail> LRemoveAtOp<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = Tail;
}

impl<Target, Index, NonTarget, Tail> LRemoveAtOp<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LRemoveAtOp<Target, Index>,
{
    type Output = LCons<NonTarget, LRemoveAtOpOutput<Tail, Target, Index>>;
}

pub type LRemoveAtOpOutput<List, Target, Index> = <List as LRemoveAtOp<Target, Index>>::Output;

/// A [Functor] that removes `Target` from [TList].
pub struct LRemoveAtFunctor<Target, Index> {
    _phantom: PhantomData<(Target, Index)>,
}

pub type LRemoveAt<List, Target, Index> = ApplyFunctor<LRemoveAtFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for LRemoveAtFunctor<Target, Index>
where
    List: TList + LRemoveAtOp<Target, Index>,
    Index: Counter,
{
    type Output = LRemoveAtOpOutput<List, Target, Index>;
}

// remove multiple items

/// Removes a collection of types from [TList].
///
/// The `Targets` argument accepts a [TList] of types to be removed.
/// The `Indexes` argument can be left unspecified.
pub trait LRemoveManyOp<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<List> LRemoveManyOp<LNil, LNil> for List
where
    List: TList,
{
    type Output = List;
}

impl<Index, IRemain, Target, TRemain, Head, Tail>
    LRemoveManyOp<LCons<Target, TRemain>, LCons<Index, IRemain>> for LCons<Head, Tail>
where
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Tail: TList,
    Self: LRemoveAtOp<Target, Index>,
    LRemoveAtOpOutput<Self, Target, Index>: LRemoveManyOp<TRemain, IRemain>,
{
    type Output = LRemoveManyOpOutput<LRemoveAtOpOutput<Self, Target, Index>, TRemain, IRemain>;
}

pub type LRemoveManyOpOutput<List, Targets, Indexes> =
    <List as LRemoveManyOp<Targets, Indexes>>::Output;

/// A [Functor] that removes multiple `Targets` in [TList].
pub struct LRemoveManyFunctor<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(Targets, Indexes)>,
}

pub type LRemoveMany<List, Targets, Indexes> =
    ApplyFunctor<LRemoveManyFunctor<Targets, Indexes>, List>;

impl<List, Targets, Indexes> Functor<List> for LRemoveManyFunctor<Targets, Indexes>
where
    List: TList + LRemoveManyOp<Targets, Indexes>,
    Targets: TList,
    Indexes: TList,
{
    type Output = LRemoveManyOpOutput<List, Targets, Indexes>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, TListType};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;

    type SomeList = TListType![A, B, C];

    // remove
    type Assert7<Idx> = AssertSame<LRemoveAt<SomeList, B, Idx>, TListType![A, C]>;

    // remove multiple items
    type Assert8<Idx> = AssertSame<LRemoveMany<SomeList, TListType![A, C], Idx>, TListType![B]>;

    // remove until empty
    type Assert9<Idx> = AssertSame<LRemoveMany<SomeList, TListType![B, A, C], Idx>, TListType![]>;

    #[test]
    fn tlist_remove_test() {
        let _: Assert7<_> = ();
        let _: Assert8<_> = ();
        let _: Assert9<_> = ();
    }
}
