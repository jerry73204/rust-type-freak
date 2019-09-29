use super::{KVCons, KVList};
use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
    list::{
        LIndexOfManyOp, LIndexOfManyOpOutput, LIndexOfOp, LIndexOfOpOutput, LUnzip, LUnzipFunctor,
        LUnzipOp, LUnzipOpFormerOutput, TList,
    },
    tuple::{FirstOf, FirstOfFunctor, SecondOf, SecondOfFunctor},
};
use std::marker::PhantomData;
use typenum::Unsigned;

/// A functor that gets index of `Target` in [KVList].
pub struct KVIndexOfFunctor<Target, Index>
where
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type KVIndexOf<List, Target, Index> = ApplyFunctor<KVIndexOfFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for KVIndexOfFunctor<Target, Index>
where
    List: KVList + LUnzipOp,
    Index: Counter,
    LUnzipOpFormerOutput<List>: LIndexOfOp<Target, Index>,
    LIndexOfOpOutput<LUnzipOpFormerOutput<List>, Target, Index>: Unsigned,
{
    type Output = LIndexOfOpOutput<LUnzipOpFormerOutput<List>, Target, Index>;
}

// index of many

/// A functor that gets multiple indexes of `Targets` in [KVList].
pub struct KVIndexOfManyFunctor<Targets, Indexes> {
    _phantom: PhantomData<(Targets, Indexes)>,
}

impl<List, Targets, Indexes> Functor<List> for KVIndexOfManyFunctor<Targets, Indexes>
where
    List: KVList + LUnzipOp,
    Targets: TList,
    Indexes: TList,
    LUnzipOpFormerOutput<List>: LIndexOfManyOp<Targets, Indexes>,
    LIndexOfManyOpOutput<LUnzipOpFormerOutput<List>, Targets, Indexes>: TList,
{
    type Output = LIndexOfManyOpOutput<LUnzipOpFormerOutput<List>, Targets, Indexes>;
}

pub type KVIndexOfMany<List, Targets, Indexes> =
    ApplyFunctor<KVIndexOfManyFunctor<Targets, Indexes>, List>;

// get key-value pair

/// A functor that gets key-value pair from [KVList].
pub struct KVKeyValueAtFunctor<Target, Index> {
    _phantom: PhantomData<(Target, Index)>,
}

pub type KVKeyValueAt<List, Target, Index> = ApplyFunctor<KVKeyValueAtFunctor<Target, Index>, List>;

impl<Target, Value, Tail> Functor<KVCons<Target, Value, Tail>>
    for KVKeyValueAtFunctor<Target, Current>
where
    Tail: KVList,
{
    type Output = (Target, Value);
}

impl<NonTarget, Value, Tail, Target, Index> Functor<KVCons<NonTarget, Value, Tail>>
    for KVKeyValueAtFunctor<Target, Next<Index>>
where
    Tail: KVList,
    Index: Counter,
    KVKeyValueAtFunctor<Target, Index>: Functor<Tail>,
{
    type Output = KVKeyValueAt<Tail, Target, Index>;
}

// get value of key

/// A functor that gets the value at `Target` in [KVList].
pub struct KVValueAtFunctor<Target, Index>
where
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type KVValueAt<List, Target, Index> = ApplyFunctor<KVValueAtFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for KVValueAtFunctor<Target, Index>
where
    List: KVList,
    Index: Counter,
    KVKeyValueAtFunctor<Target, Index>: Functor<List>,
    SecondOfFunctor: Functor<KVKeyValueAt<List, Target, Index>>,
{
    type Output = SecondOf<KVKeyValueAt<List, Target, Index>>;
}

/// A [Functor] that extracts all keys from [KVList].
pub struct KVKeysFunctor;

impl<List> Functor<List> for KVKeysFunctor
where
    List: KVList,
    LUnzipFunctor: Functor<List>,
    FirstOfFunctor: Functor<LUnzip<List>>,
    FirstOf<LUnzip<List>>: TList,
{
    type Output = FirstOf<LUnzip<List>>;
}

/// A [Functor] that extracts all values from [KVList].
pub struct KVValuesFunctor;

impl<List> Functor<List> for KVValuesFunctor
where
    List: KVList,
    LUnzipFunctor: Functor<List>,
    SecondOfFunctor: Functor<LUnzip<List>>,
    SecondOf<LUnzip<List>>: TList,
{
    type Output = SecondOf<LUnzip<List>>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, KVListType, TListType};
    use typenum::consts::*;

    type AssertEqual<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;

    struct Va;
    struct Vb;
    struct Vc;

    type SomeList = KVListType![(A, Va), (B, Vb), (C, Vc)];

    // concat
    type Assert13<Idx> =
        AssertEqual<KVIndexOfMany<SomeList, TListType![C, A, B], Idx>, TListType![U2, U0, U1]>;

    // index of
    type Assert14<Idx> = AssertEqual<KVIndexOf<SomeList, A, Idx>, U0>;
    type Assert15<Idx> = AssertEqual<KVIndexOf<SomeList, B, Idx>, U1>;
    type Assert16<Idx> = AssertEqual<KVIndexOf<SomeList, C, Idx>, U2>;

    // get key-value pair
    type Assert17<Idx> = AssertEqual<KVKeyValueAt<SomeList, B, Idx>, (B, Vb)>;

    // get value pair
    type Assert18<Idx> = AssertEqual<KVValueAt<SomeList, B, Idx>, Vb>;

    #[test]
    fn kvlist_access_test() {
        let _: Assert13<_> = ();
        let _: Assert14<_> = ();
        let _: Assert15<_> = ();
        let _: Assert16<_> = ();
        let _: Assert17<_> = ();
        let _: Assert18<_> = ();
    }
}
