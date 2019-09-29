//! A typed list of key-value pairs.

mod macros;
pub mod marker;

use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
    list::{
        LAppend, LAppendFunctor, LConcatOp, LConcatOpOutput, LCons, LIndexOfManyOp,
        LIndexOfManyOpOutput, LIndexOfOp, LIndexOfOpOutput, LLengthOp, LLengthOpOutput, LNil,
        LPrepend, LReverse, LReverseFunctor, LSetEqualOp, LSetEqualOpOutput, LUnzipOp,
        LUnzipOpFormerOutput, TList,
    },
    tuple::{SecondOf, SecondOfFunctor},
};
use std::marker::PhantomData;
use typenum::Unsigned;

// list

/// The trait represents a list of key-value pairs.
pub trait KVList
where
    Self: TList,
{
}

/// A node of [KVList].
pub type KVCons<Key, Value, Tail> = LCons<(Key, Value), Tail>;

impl<Key, Value, Tail> KVList for KVCons<Key, Value, Tail> where Tail: KVList {}

/// The ending node of [KVList].
pub type KVNil = LNil;

impl KVList for LNil {}

// length of list

/// A functor that gets length of [KVList].
pub struct KVLengthFunctor;

impl<List> Functor<List> for KVLengthFunctor
where
    List: KVList + LLengthOp,
    LLengthOpOutput<List>: Unsigned,
{
    type Output = LLengthOpOutput<List>;
}

pub type KVLength<List> = ApplyFunctor<KVLengthFunctor, List>;

// prepend

/// A functor that prepends a key-value pair to [KVList].
pub struct KVPrependFunctor<Key, Value> {
    _phantom: PhantomData<(Key, Value)>,
}

impl<List, Key, Value> Functor<List> for KVPrependFunctor<Key, Value>
where
    List: KVList,
    LAppendFunctor<(Key, Value)>: Functor<List>,
{
    type Output = LPrepend<List, (Key, Value)>;
}

pub type KVPrepend<List, Key, Value> = ApplyFunctor<KVPrependFunctor<Key, Value>, List>;

// append

/// A functor that appends a key-value pair to [KVList].
pub struct KVAppendFunctor<Key, Value> {
    _phantom: PhantomData<(Key, Value)>,
}

impl<List, Key, Value> Functor<List> for KVAppendFunctor<Key, Value>
where
    List: KVList,
    LAppendFunctor<(Key, Value)>: Functor<List>,
{
    type Output = LAppend<List, (Key, Value)>;
}

pub type KVAppend<List, Key, Value> = ApplyFunctor<KVAppendFunctor<Key, Value>, List>;

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

/// A functor that inserts `Key`-`Value` pair into [KVList] at `Target`.
pub struct KVInsertAtFunctor<Key, Value, Target, Index> {
    _phantom: PhantomData<(Key, Value, Target, Index)>,
}

pub type KVInsertAt<List, Key, Value, Target, Index> =
    ApplyFunctor<KVInsertAtFunctor<Key, Value, Target, Index>, List>;

impl<List, Key, Value, Target, Index> Functor<List> for KVInsertAtFunctor<Key, Value, Target, Index>
where
    List: KVList + KVInsertAtOp<Key, Value, Target, Index>,
    Index: Counter,
{
    type Output = KVInsertAtOpOutput<List, Key, Value, Target, Index>;
}

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

/// A functor that removes `Target` from `KVList`.
pub struct KVRemoveAtFunctor<Target, Index> {
    _phantom: PhantomData<(Target, Index)>,
}

pub type KVRemoveAt<List, Target, Index> = ApplyFunctor<KVRemoveAtFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for KVRemoveAtFunctor<Target, Index>
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

/// A functor that removes multiple `Targets` from [KVList].
pub struct KVRemoveManyFunctor<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(Targets, Indexes)>,
}

pub type KVRemoveMany<List, Targets, Indexes> =
    ApplyFunctor<KVRemoveManyFunctor<Targets, Indexes>, List>;

impl<List, Targets, Indexes> Functor<List> for KVRemoveManyFunctor<Targets, Indexes>
where
    List: KVList + KVRemoveManyOp<Targets, Indexes>,
    Targets: TList,
    Indexes: TList,
{
    type Output = KVRemoveManyOpOutput<List, Targets, Indexes>;
}

// index of item

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

// reverse

/// A [Functor] that reverses a [KVList].
pub struct KVReverseFuntor {}

pub type KVReverse<List> = ApplyFunctor<KVReverseFuntor, List>;

impl<List> Functor<List> for KVReverseFuntor
where
    List: KVList,
    LReverse<List>: TList,
    LReverseFunctor: Functor<List>,
{
    type Output = LReverse<List>;
}

// set equal

/// A functor that compares if two [KVList]s have same set of keys.
pub struct KVSetEqualFuntor<Rhs, Indexes>
where
    Rhs: KVList,
    Indexes: TList,
{
    _phantom: PhantomData<(Rhs, Indexes)>,
}

pub type KVSetEqual<Lhs, Rhs, Indexes> = ApplyFunctor<KVSetEqualFuntor<Rhs, Indexes>, Lhs>;

impl<Lhs, Rhs, Indexes> Functor<Lhs> for KVSetEqualFuntor<Rhs, Indexes>
where
    Lhs: KVList + LUnzipOp,
    Rhs: KVList + LUnzipOp,
    Indexes: TList,
    LUnzipOpFormerOutput<Lhs>: LSetEqualOp<LUnzipOpFormerOutput<Rhs>, Indexes>,
{
    type Output = LSetEqualOpOutput<LUnzipOpFormerOutput<Lhs>, LUnzipOpFormerOutput<Rhs>, Indexes>;
}

// concatenate

/// A [Functor] that concatenates input and `Rhs` [KVList]s.
pub struct KVConcatFunctor<Rhs>
where
    Rhs: KVList,
{
    _phantom: PhantomData<Rhs>,
}

pub type KVConcat<Lhs, Rhs> = ApplyFunctor<KVConcatFunctor<Rhs>, Lhs>;

impl<Lhs, Rhs> Functor<Lhs> for KVConcatFunctor<Rhs>
where
    Lhs: KVList + LConcatOp<Rhs>,
    Rhs: KVList,
    LConcatOpOutput<Lhs, Rhs>: KVList,
{
    type Output = LConcatOpOutput<Lhs, Rhs>;
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, KVListType, TListType};
    use typenum::consts::*;

    type AssertEqual<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    struct Va;
    struct Vb;
    struct Vc;
    struct Vd;
    struct Ve;

    type EmptyList = KVListType![];
    type SomeList = KVListType![(A, Va), (B, Vb), (C, Vc)];
    type AnotherList = KVListType![(D, Vd), (E, Ve)];

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

    // remove
    type Assert7<Idx> = AssertEqual<KVRemoveAt<SomeList, B, Idx>, KVListType![(A, Va), (C, Vc)]>;

    // remove multiple items
    type Assert8<Idx> =
        AssertEqual<KVRemoveMany<SomeList, TListType![A, C], Idx>, KVListType![(B, Vb)]>;

    // remove until empty
    type Assert9<Idx> =
        AssertEqual<KVRemoveMany<SomeList, TListType![B, A, C], Idx>, KVListType![]>;

    // reverse list
    type Assert10 = AssertEqual<KVReverse<SomeList>, KVListType![(C, Vc), (B, Vb), (A, Va)]>;

    // assert identical set of items
    type Assert11<Idx> = KVSetEqual<SomeList, KVListType![(C, Vc), (A, Va), (B, Vb)], Idx>;

    // concat
    type Assert12 = AssertEqual<
        KVConcat<SomeList, AnotherList>,
        KVListType![(A, Va), (B, Vb), (C, Vc), (D, Vd), (E, Ve)],
    >;

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
    fn tlist_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
        let _: Assert7<_> = ();
        let _: Assert8<_> = ();
        let _: Assert9<_> = ();
        let _: Assert10 = ();
        let _: Assert11<_> = ();
        let _: Assert12 = ();
        let _: Assert13<_> = ();
        let _: Assert14<_> = ();
        let _: Assert15<_> = ();
        let _: Assert16<_> = ();
        let _: Assert17<_> = ();
        let _: Assert18<_> = ();
    }
}
