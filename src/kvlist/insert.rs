use super::{KVCons, KVList};
use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyMap, Map},
    list::{LAppend, LAppendMap, LPrepend},
};
use std::marker::PhantomData;

// prepend

/// A map that prepends a key-value pair to [KVList].
pub struct KVPrependMap<Key, Value> {
    _phantom: PhantomData<(Key, Value)>,
}

impl<List, Key, Value> Map<List> for KVPrependMap<Key, Value>
where
    List: KVList,
    LAppendMap<(Key, Value)>: Map<List>,
{
    type Output = LPrepend<List, (Key, Value)>;
}

pub type KVPrepend<List, Key, Value> = ApplyMap<KVPrependMap<Key, Value>, List>;

// append

/// A map that appends a key-value pair to [KVList].
pub struct KVAppendMap<Key, Value> {
    _phantom: PhantomData<(Key, Value)>,
}

impl<List, Key, Value> Map<List> for KVAppendMap<Key, Value>
where
    List: KVList,
    LAppendMap<(Key, Value)>: Map<List>,
{
    type Output = LAppend<List, (Key, Value)>;
}

pub type KVAppend<List, Key, Value> = ApplyMap<KVAppendMap<Key, Value>, List>;

// insert at

/// A type operator that inserts `Key`-`Value` pair into [KVList] at `Target`.
pub trait KVInsertAtOp<Key, Value, Target, Index>
where
    Index: Counter,
    Self: KVList,
    Self::Output: KVList,
{
    type Output;
}

pub type KVInsertAtOpOutput<List, Key, Value, Target, Index> =
    <List as KVInsertAtOp<Key, Value, Target, Index>>::Output;

impl<Key, Value, Target, TargetValue, Tail> KVInsertAtOp<Key, Value, Target, Current>
    for KVCons<Target, TargetValue, Tail>
where
    Tail: KVList,
{
    type Output = KVCons<Key, Value, KVCons<Target, TargetValue, Tail>>;
}
impl<Key, Value, Target, Index, NonTarget, NonTargetValue, Tail>
    KVInsertAtOp<Key, Value, Target, Next<Index>> for KVCons<NonTarget, NonTargetValue, Tail>
where
    Tail: KVList + KVInsertAtOp<Key, Value, Target, Index>,
    Index: Counter,
{
    type Output =
        KVCons<NonTarget, NonTargetValue, KVInsertAtOpOutput<Tail, Key, Value, Target, Index>>;
}

/// A map that inserts `Key`-`Value` pair into [KVList] at `Target`.
pub struct KVInsertAtMap<Key, Value, Target, Index> {
    _phantom: PhantomData<(Key, Value, Target, Index)>,
}

pub type KVInsertAt<List, Key, Value, Target, Index> =
    ApplyMap<KVInsertAtMap<Key, Value, Target, Index>, List>;

impl<List, Key, Value, Target, Index> Map<List> for KVInsertAtMap<Key, Value, Target, Index>
where
    List: KVList + KVInsertAtOp<Key, Value, Target, Index>,
    Index: Counter,
{
    type Output = KVInsertAtOpOutput<List, Key, Value, Target, Index>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, KVListType};

    type AssertEqual<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;
    struct D;

    struct Va;
    struct Vb;
    struct Vc;
    struct Vd;

    type EmptyList = KVListType![];
    type SomeList = KVListType![(A, Va), (B, Vb), (C, Vc)];

    // prepend empty list
    type Assert1 = AssertEqual<KVPrepend<EmptyList, A, Va>, KVListType![(A, Va)]>;

    // append empty list
    type Assert2 = AssertEqual<KVAppend<EmptyList, D, Vd>, KVListType![(D, Vd)]>;

    // prepend non-empty list
    type Assert3 =
        AssertEqual<KVPrepend<SomeList, D, Vd>, KVListType![(D, Vd), (A, Va), (B, Vb), (C, Vc)]>;

    // append non-empty list
    type Assert4 =
        AssertEqual<KVAppend<SomeList, D, Vd>, KVListType![(A, Va), (B, Vb), (C, Vc), (D, Vd)]>;

    // insert in middle
    type Assert5<Idx> = AssertEqual<
        KVInsertAt<SomeList, D, Vd, B, Idx>,
        KVListType![(A, Va), (D, Vd), (B, Vb), (C, Vc)],
    >;

    // insert at end
    type Assert6<Idx> = AssertEqual<
        KVInsertAt<SomeList, D, Vd, C, Idx>,
        KVListType![(A, Va), (B, Vb), (D, Vd), (C, Vc)],
    >;

    #[test]
    fn kvlist_insert_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
    }
}
