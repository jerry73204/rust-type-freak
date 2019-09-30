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
pub struct KVGetKeyValueAtFunctor<Target, Index> {
    _phantom: PhantomData<(Target, Index)>,
}

pub type KVGetKeyValueAt<List, Target, Index> =
    ApplyFunctor<KVGetKeyValueAtFunctor<Target, Index>, List>;

impl<Target, Value, Tail> Functor<KVCons<Target, Value, Tail>>
    for KVGetKeyValueAtFunctor<Target, Current>
where
    Tail: KVList,
{
    type Output = (Target, Value);
}

impl<NonTarget, Value, Tail, Target, Index> Functor<KVCons<NonTarget, Value, Tail>>
    for KVGetKeyValueAtFunctor<Target, Next<Index>>
where
    Tail: KVList,
    Index: Counter,
    KVGetKeyValueAtFunctor<Target, Index>: Functor<Tail>,
{
    type Output = KVGetKeyValueAt<Tail, Target, Index>;
}

// get value of key

/// A functor that gets the value at `Target` in [KVList].
pub struct KVGetValueAtFunctor<Target, Index>
where
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type KVGetValueAt<List, Target, Index> = ApplyFunctor<KVGetValueAtFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for KVGetValueAtFunctor<Target, Index>
where
    List: KVList,
    Index: Counter,
    KVGetKeyValueAtFunctor<Target, Index>: Functor<List>,
    SecondOfFunctor: Functor<KVGetKeyValueAt<List, Target, Index>>,
{
    type Output = SecondOf<KVGetKeyValueAt<List, Target, Index>>;
}

pub trait KVSetValueAtOp<NewValue, Target, Index>
where
    Index: Counter,
    Self: KVList,
    Self::Output: KVList,
{
    type Output;
}

/// A type operator that sets the value at `Target` in [KVList].
pub type KVSetValueAtOpOutput<List, NewValue, Target, Index> =
    <List as KVSetValueAtOp<NewValue, Target, Index>>::Output;

impl<NewValue, Target, OldValue, Tail> KVSetValueAtOp<NewValue, Target, Current>
    for KVCons<Target, OldValue, Tail>
where
    Tail: KVList,
{
    type Output = KVCons<Target, NewValue, Tail>;
}

impl<NewValue, Target, Index, NonTarget, Value, Tail> KVSetValueAtOp<NewValue, Target, Next<Index>>
    for KVCons<NonTarget, Value, Tail>
where
    Tail: KVList + KVSetValueAtOp<NewValue, Target, Index>,
    Index: Counter,
{
    type Output = KVCons<NonTarget, Value, KVSetValueAtOpOutput<Tail, NewValue, Target, Index>>;
}

/// A [Functor] that sets the value at `Target` in [KVList].
pub struct KVSetValueAtFunctor<NewValue, Target, Index>
where
    Index: Counter,
{
    _phantom: PhantomData<(NewValue, Target, Index)>,
}

pub type KVSetValueAt<List, NewValue, Target, Index> =
    ApplyFunctor<KVSetValueAtFunctor<NewValue, Target, Index>, List>;

impl<List, NewValue, Target, Index> Functor<List> for KVSetValueAtFunctor<NewValue, Target, Index>
where
    List: KVList + KVSetValueAtOp<NewValue, Target, Index>,
    Index: Counter,
{
    type Output = KVSetValueAtOpOutput<List, NewValue, Target, Index>;
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
    struct Vx;

    type SomeList = KVListType![(A, Va), (B, Vb), (C, Vc)];

    // concat
    type Assert1<Idx> =
        AssertEqual<KVIndexOfMany<SomeList, TListType![C, A, B], Idx>, TListType![U2, U0, U1]>;

    // index of
    type Assert2<Idx> = AssertEqual<KVIndexOf<SomeList, A, Idx>, U0>;
    type Assert3<Idx> = AssertEqual<KVIndexOf<SomeList, B, Idx>, U1>;
    type Assert4<Idx> = AssertEqual<KVIndexOf<SomeList, C, Idx>, U2>;

    // get key-value pair
    type Assert5<Idx> = AssertEqual<KVGetKeyValueAt<SomeList, B, Idx>, (B, Vb)>;

    // get value pair
    type Assert6<Idx> = AssertEqual<KVGetValueAt<SomeList, B, Idx>, Vb>;

    // set value
    type Assert7<Idx> =
        AssertEqual<KVSetValueAt<SomeList, Vx, B, Idx>, KVListType![(A, Va), (B, Vx), (C, Vc)]>;

    #[test]
    fn kvlist_access_test() {
        let _: Assert1<_> = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_> = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
        let _: Assert7<_> = ();
    }
}
