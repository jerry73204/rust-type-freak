use super::{KVCons, KVList};
use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyMap, Map},
    list::{LCons, LNil, TList},
};
use std::marker::PhantomData;

// remove

/// A type operator that removes `Target` from [KVList].
pub trait KVRemoveAtOp<Target, Index>
where
    Index: Counter,
    Self: KVList,
    Self::Output: KVList,
{
    type Output;
}

impl<Target, Value, Tail> KVRemoveAtOp<Target, Current> for KVCons<Target, Value, Tail>
where
    Tail: KVList,
{
    type Output = Tail;
}

impl<Target, Index, NonTarget, Value, Tail> KVRemoveAtOp<Target, Next<Index>>
    for KVCons<NonTarget, Value, Tail>
where
    Index: Counter,
    Tail: KVList + KVRemoveAtOp<Target, Index>,
{
    type Output = KVCons<NonTarget, Value, KVRemoveAtOpOutput<Tail, Target, Index>>;
}

pub type KVRemoveAtOpOutput<KVist, Target, Index> = <KVist as KVRemoveAtOp<Target, Index>>::Output;

/// A map that removes `Target` from `KVList`.
pub struct KVRemoveAtMap<Target, Index> {
    _phantom: PhantomData<(Target, Index)>,
}

pub type KVRemoveAt<List, Target, Index> = ApplyMap<KVRemoveAtMap<Target, Index>, List>;

impl<List, Target, Index> Map<List> for KVRemoveAtMap<Target, Index>
where
    List: KVList + KVRemoveAtOp<Target, Index>,
    Index: Counter,
{
    type Output = KVRemoveAtOpOutput<List, Target, Index>;
}

// remove multiple items

/// A type operator that removes multiple `Targets` from [KVList].
pub trait KVRemoveManyOp<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: KVList,
    Self::Output: KVList,
{
    type Output;
}

impl<List> KVRemoveManyOp<LNil, LNil> for List
where
    List: KVList,
{
    type Output = List;
}

impl<Index, IRemain, Target, TRemain, Key, Value, Tail>
    KVRemoveManyOp<LCons<Target, TRemain>, LCons<Index, IRemain>> for KVCons<Key, Value, Tail>
where
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Tail: KVList,
    Self: KVRemoveAtOp<Target, Index>,
    KVRemoveAtOpOutput<Self, Target, Index>: KVRemoveManyOp<TRemain, IRemain>,
{
    type Output = KVRemoveManyOpOutput<KVRemoveAtOpOutput<Self, Target, Index>, TRemain, IRemain>;
}

pub type KVRemoveManyOpOutput<KVist, Targets, Indexes> =
    <KVist as KVRemoveManyOp<Targets, Indexes>>::Output;

/// A map that removes multiple `Targets` from [KVList].
pub struct KVRemoveManyMap<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(Targets, Indexes)>,
}

pub type KVRemoveMany<List, Targets, Indexes> = ApplyMap<KVRemoveManyMap<Targets, Indexes>, List>;

impl<List, Targets, Indexes> Map<List> for KVRemoveManyMap<Targets, Indexes>
where
    List: KVList + KVRemoveManyOp<Targets, Indexes>,
    Targets: TList,
    Indexes: TList,
{
    type Output = KVRemoveManyOpOutput<List, Targets, Indexes>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, KVListType, TListType};

    type AssertEqual<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;

    struct Va;
    struct Vb;
    struct Vc;

    type SomeList = KVListType![(A, Va), (B, Vb), (C, Vc)];

    // remove
    type Assert7<Idx> = AssertEqual<KVRemoveAt<SomeList, B, Idx>, KVListType![(A, Va), (C, Vc)]>;

    // remove multiple items
    type Assert8<Idx> =
        AssertEqual<KVRemoveMany<SomeList, TListType![A, C], Idx>, KVListType![(B, Vb)]>;

    // remove until empty
    type Assert9<Idx> =
        AssertEqual<KVRemoveMany<SomeList, TListType![B, A, C], Idx>, KVListType![]>;

    #[test]
    fn kvlist_remove_test() {
        let _: Assert7<_> = ();
        let _: Assert8<_> = ();
        let _: Assert9<_> = ();
    }
}
