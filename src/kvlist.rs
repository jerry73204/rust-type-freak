use crate::{
    counter::{Counter, Current, Next},
    list::{LCons, LNil, TList},
};
use std::{marker::PhantomData, ops::Add};
use typenum::{Sum, Unsigned, U0, U1};

// list

/// The trait represents a list of key-value pairs.
pub trait KVList {}

/// A node of [KVList].
pub struct KVCons<Key, Value, Tail>
where
    Tail: KVList,
{
    _phantom: PhantomData<(Key, Value, Tail)>,
}

impl<Key, Value, Tail> KVCons<Key, Value, Tail>
where
    Tail: KVList,
{
    pub fn new() -> KVCons<Key, Value, Tail> {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Key, Value, Tail> KVList for KVCons<Key, Value, Tail> where Tail: KVList {}

/// The ending node of [KVList].
pub struct KVNil;

impl KVList for KVNil {}

// {,non-}empty list trait

/// A marker trait that marks the empty [KVList].
pub trait EmptyKVList: KVList {}

impl EmptyKVList for KVNil {}

/// A marker trait that marks non-empty [KVList].
pub trait NonEmptyKVList: KVList {}

impl<Key, Value, Tail> NonEmptyKVList for KVCons<Key, Value, Tail> where Tail: KVList {}

// length of list

/// A type operator that gets length of [KVList].
pub trait KVLength
where
    Self: KVList,
    Self::Out: Unsigned,
{
    type Out;
}

impl KVLength for KVNil {
    type Out = U0;
}

impl<Key, Value, Tail> KVLength for KVCons<Key, Value, Tail>
where
    Tail: KVList + KVLength,
    KVLengthOut<Tail>: Add<U1>,
    Sum<KVLengthOut<Tail>, U1>: Unsigned,
{
    type Out = Sum<KVLengthOut<Tail>, U1>;
}

pub type KVLengthOut<KVist> = <KVist as KVLength>::Out;

// prepend

/// A type operator that prepends a key-value pair to [KVList].
pub trait KVPrepend<Key, Value>
where
    Self: KVList,
{
    type Out;
}

impl<Key, Value, List> KVPrepend<Key, Value> for List
where
    List: KVList,
{
    type Out = KVCons<Key, Value, List>;
}

pub type KVPrependOut<KVist, Key, Value> = <KVist as KVPrepend<Key, Value>>::Out;

// append

/// A type operator that appends a key-value pair to [KVList].
pub trait KVAppend<Key, Value>
where
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

impl<Key, Value> KVAppend<Key, Value> for KVNil {
    type Out = KVCons<Key, Value, KVNil>;
}

impl<NewKey, NewValue, Key, Value, Tail> KVAppend<NewKey, NewValue> for KVCons<Key, Value, Tail>
where
    Tail: KVList + KVAppend<NewKey, NewValue>,
    KVAppendOut<Tail, NewKey, NewValue>: KVList,
{
    type Out = KVCons<Key, Value, KVAppendOut<Tail, NewKey, NewValue>>;
}

pub type KVAppendOut<KVist, NewKey, NewValue> = <KVist as KVAppend<NewKey, NewValue>>::Out;

// insert at

/// A type operator that inserts a key-value pair into [KVList] at specific key.
pub trait KVInsertAt<Key, Value, Target, Index>
where
    Index: Counter,
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

impl<Key, Value, Target, TargetValue, Tail> KVInsertAt<Key, Value, Target, Current>
    for KVCons<Target, TargetValue, Tail>
where
    Tail: KVList,
{
    type Out = KVCons<Target, TargetValue, KVCons<Key, Value, Tail>>;
}

impl<NewKey, NewValue, Target, Index, Key, Value, Tail>
    KVInsertAt<NewKey, NewValue, Target, Next<Index>> for KVCons<Key, Value, Tail>
where
    Tail: KVList + KVInsertAt<NewKey, NewValue, Target, Index>,
    Index: Counter,
{
    type Out = KVCons<Key, Value, KVInsertAtOut<Tail, NewKey, NewValue, Target, Index>>;
}

pub type KVInsertAtOut<KVist, NewKey, NewValue, Target, Index> =
    <KVist as KVInsertAt<NewKey, NewValue, Target, Index>>::Out;

// remove

pub trait KVRemoveAt<Target, Index>
where
    Index: Counter,
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

impl<Target, Value, Tail> KVRemoveAt<Target, Current> for KVCons<Target, Value, Tail>
where
    Tail: KVList,
{
    type Out = Tail;
}

impl<Target, Index, NonTarget, Value, Tail> KVRemoveAt<Target, Next<Index>>
    for KVCons<NonTarget, Value, Tail>
where
    Index: Counter,
    Tail: KVList + KVRemoveAt<Target, Index>,
{
    type Out = KVCons<NonTarget, Value, KVRemoveAtOut<Tail, Target, Index>>;
}

pub type KVRemoveAtOut<KVist, Target, Index> = <KVist as KVRemoveAt<Target, Index>>::Out;

// remove multiple items

pub trait KVRemoveMany<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

impl<List> KVRemoveMany<LNil, LNil> for List
where
    List: KVList,
{
    type Out = List;
}

impl<Index, IRemain, Target, TRemain, Key, Value, Tail>
    KVRemoveMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for KVCons<Key, Value, Tail>
where
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Tail: KVList,
    Self: KVRemoveAt<Target, Index>,
    KVRemoveAtOut<Self, Target, Index>: KVRemoveMany<TRemain, IRemain>,
{
    type Out = KVRemoveManyOut<KVRemoveAtOut<Self, Target, Index>, TRemain, IRemain>;
}

pub type KVRemoveManyOut<KVist, Targets, Indexes> = <KVist as KVRemoveMany<Targets, Indexes>>::Out;

// index of item

pub trait KVIndexOf<Target, Index>
where
    Self: KVList,
    Index: Counter,
{
    const INDEX: usize;
}

impl<Target, Value, Tail> KVIndexOf<Target, Current> for KVCons<Target, Value, Tail>
where
    Tail: KVList,
{
    const INDEX: usize = 0;
}

impl<Target, Index, NonTarget, Value, Tail> KVIndexOf<Target, Next<Index>>
    for KVCons<NonTarget, Value, Tail>
where
    Index: Counter,
    Tail: KVList + KVIndexOf<Target, Index>,
{
    const INDEX: usize = 1 + <Tail as KVIndexOf<Target, Index>>::INDEX;
}

// index of many

pub trait KVIndexOfMany<Targets, Indexes>
where
    Self: KVList,
    Targets: TList,
    Indexes: TList,
{
    fn indexes() -> Vec<usize>;
    fn inverse_indexes() -> Vec<usize>;
}

impl<List> KVIndexOfMany<LNil, LNil> for List
where
    List: KVList + KVLength,
{
    fn indexes() -> Vec<usize> {
        vec![]
    }

    fn inverse_indexes() -> Vec<usize> {
        (0..KVLengthOut::<List>::USIZE).collect()
    }
}

impl<Index, IRemain, Target, TRemain, Key, Value, Tail>
    KVIndexOfMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for KVCons<Key, Value, Tail>
where
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Tail: KVList,
    Self: KVIndexOf<Target, Index> + KVIndexOfMany<TRemain, IRemain>,
{
    fn indexes() -> Vec<usize> {
        let mut indexes = <Self as KVIndexOfMany<TRemain, IRemain>>::indexes();
        indexes.insert(0, <Self as KVIndexOf<Target, Index>>::INDEX);
        indexes
    }

    fn inverse_indexes() -> Vec<usize> {
        let mut indexes = <Self as KVIndexOfMany<TRemain, IRemain>>::inverse_indexes();
        indexes.remove_item(&<Self as KVIndexOf<Target, Index>>::INDEX);
        indexes
    }
}

// reverse

pub trait KVReverseWithTail<Tail>
where
    Tail: KVList,
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

impl<Tail> KVReverseWithTail<Tail> for KVNil
where
    Tail: KVList,
{
    type Out = Tail;
}

impl<ReversedTail, Key, Value, Tail> KVReverseWithTail<ReversedTail> for KVCons<Key, Value, Tail>
where
    ReversedTail: KVList,
    Tail: KVList + KVReverseWithTail<KVCons<Key, Value, ReversedTail>>,
{
    type Out = KVReverseWithTailOut<Tail, KVCons<Key, Value, ReversedTail>>;
}

pub type KVReverseWithTailOut<KVist, ReversedTail> =
    <KVist as KVReverseWithTail<ReversedTail>>::Out;
pub type KVReverseOut<KVist> = KVReverseWithTailOut<KVist, KVNil>;

// set equal

pub trait KVSetEqual<Rhs, Indexes>
where
    Rhs: KVList,
    Indexes: TList,
    Self: KVList,
{
    type Out;
}

impl KVSetEqual<KVNil, LNil> for KVNil {
    type Out = ();
}

impl<LKey, LValue, LTail, RKey, RValue, RTail, Index, IRemain>
    KVSetEqual<KVCons<RKey, RValue, RTail>, LCons<Index, IRemain>> for KVCons<LKey, LValue, LTail>
where
    Index: Counter,
    IRemain: TList,
    LTail: KVList,
    RTail: KVList,
    Self: KVRemoveAt<RKey, Index>,
    KVRemoveAtOut<Self, RKey, Index>: KVSetEqual<RTail, IRemain>,
{
    type Out = KVSetEqualOut<KVRemoveAtOut<Self, RKey, Index>, RTail, IRemain>;
}

pub type KVSetEqualOut<KVhs, Rhs, Indexes> = <KVhs as KVSetEqual<Rhs, Indexes>>::Out;

// concatenate

pub trait KVConcat<Rhs>
where
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

impl<Rhs> KVConcat<Rhs> for KVNil
where
    Rhs: KVList,
{
    type Out = Rhs;
}

impl<Rhs, Key, Value, Tail> KVConcat<Rhs> for KVCons<Key, Value, Tail>
where
    Rhs: KVList,
    Tail: KVList + KVConcat<Rhs>,
{
    type Out = KVCons<Key, Value, KVConcatOut<Tail, Rhs>>;
}

pub type KVConcatOut<KVhs, Rhs> = <KVhs as KVConcat<Rhs>>::Out;

// combine two identical lists

pub trait KVCombineEqual<Rhs>
where
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

impl KVCombineEqual<KVNil> for KVNil {
    type Out = KVNil;
}

impl<Key, Value, LTail, RTail> KVCombineEqual<KVCons<Key, Value, RTail>>
    for KVCons<Key, Value, LTail>
where
    LTail: KVList + KVCombineEqual<RTail>,
    RTail: KVList,
    KVCombineEqualOut<LTail, RTail>: KVList,
{
    type Out = KVCons<Key, Value, KVCombineEqualOut<LTail, RTail>>;
}

pub type KVCombineEqualOut<Lhs, Rhs> = <Lhs as KVCombineEqual<Rhs>>::Out;

// get value of key
// TODO test

pub trait KVGetValue<Target, Index>
where
    Self: KVList,
{
    type Out;
}

pub type KVGetValueOut<List, Target, Index> = <List as KVGetValue<Target, Index>>::Out;

impl<Target, Value, Tail> KVGetValue<Target, Current> for KVCons<Target, Value, Tail>
where
    Tail: KVList,
{
    type Out = Value;
}

impl<Target, Index, Key, Value, Tail> KVGetValue<Target, Next<Index>> for KVCons<Key, Value, Tail>
where
    Index: Counter,
    Tail: KVList + KVGetValue<Target, Index>,
{
    type Out = KVGetValueOut<Tail, Target, Index>;
}

// insert if not exists
// TODO test

pub trait KVInsertIfNotExists<Target, DefaultValue, Index>
where
    Index: Counter,
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

pub type KVInsertIfNotExistsOut<List, Target, Value, Index> =
    <List as KVInsertIfNotExists<Target, Value, Index>>::Out;

impl<Target, Value> KVInsertIfNotExists<Target, Value, Current> for KVNil {
    type Out = KVCons<Target, Value, KVNil>;
}

impl<Target, DefaultValue, Value, Tail> KVInsertIfNotExists<Target, DefaultValue, Current>
    for KVCons<Target, Value, Tail>
where
    Tail: KVList,
{
    type Out = Self;
}

impl<Target, DefaultValue, Index, Key, Value, Tail>
    KVInsertIfNotExists<Target, DefaultValue, Next<Index>> for KVCons<Key, Value, Tail>
where
    Index: Counter,
    Tail: KVList + KVInsertIfNotExists<Target, DefaultValue, Index>,
{
    type Out = KVCons<Key, Value, KVInsertIfNotExistsOut<Tail, Target, DefaultValue, Index>>;
}

// get or insert value

pub trait KVGetOrDefaultValue<Target, DefaultValue, Index>
where
    Index: Counter,
    Self: KVList,
{
    type Out;
}

pub type KVGetOrDefaultValueOut<List, Target, Value, Index> =
    <List as KVGetOrDefaultValue<Target, Value, Index>>::Out;

impl<Target, Value> KVGetOrDefaultValue<Target, Value, Current> for KVNil {
    type Out = Value;
}

impl<Target, DefaultValue, Value, Tail> KVGetOrDefaultValue<Target, DefaultValue, Current>
    for KVCons<Target, Value, Tail>
where
    Tail: KVList,
{
    type Out = Value;
}

impl<Target, DefaultValue, Index, Key, Value, Tail>
    KVGetOrDefaultValue<Target, DefaultValue, Next<Index>> for KVCons<Key, Value, Tail>
where
    Index: Counter,
    Tail: KVList + KVGetOrDefaultValue<Target, DefaultValue, Index>,
{
    type Out = KVGetOrDefaultValueOut<Tail, Target, DefaultValue, Index>;
}

// replace value

pub trait KVReplaceValue<Target, Value, Index>
where
    Index: Counter,
    Self: KVList,
    Self::Out: KVList,
{
    type Out;
}

pub type KVReplaceValueOut<List, Target, Value, Index> =
    <List as KVReplaceValue<Target, Value, Index>>::Out;

impl<Target, NewValue, OldValue, Tail> KVReplaceValue<Target, NewValue, Current>
    for KVCons<Target, OldValue, Tail>
where
    Tail: KVList,
{
    type Out = KVCons<Target, NewValue, Tail>;
}

impl<Target, NewValue, Index, Key, Value, Tail> KVReplaceValue<Target, NewValue, Next<Index>>
    for KVCons<Key, Value, Tail>
where
    Index: Counter,
    Tail: KVList + KVReplaceValue<Target, NewValue, Index>,
{
    type Out = KVCons<Key, Value, KVReplaceValueOut<Tail, Target, NewValue, Index>>;
}

// macro

#[macro_export]
macro_rules! KVListType {
    () => { $crate::kvlist::KVNil };
    (($name:ty, $value:ty)) => { $crate::kvlist::KVCons<$name, $value, $crate::kvlist::KVNil> };
    (($name:ty, $value:ty), $(($names:ty, $values:ty)),+) => { $crate::kvlist::KVCons<$name, $value, $crate::KVListType!($(($names, $values)),*)> };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfOut, KVListType, TListType};

    type AssertEqual<Lhs, Rhs> = IfOut<(), KVCombineEqualOut<Lhs, Rhs>>;

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

    type EmptyList = KVListType! {};
    type SomeList = KVListType! {(A, Va), (B, Vb), (C, Vc)};
    type AnotherList = KVListType! {(D, Vd), (E, Ve)};

    type Assert1 = AssertEqual<KVPrependOut<EmptyList, A, Va>, KVListType! {(A, Va)}>;
    type Assert2 = AssertEqual<KVAppendOut<EmptyList, D, Vd>, KVListType! {(D, Vd)}>;

    type Assert3 = AssertEqual<
        KVPrependOut<SomeList, D, Vd>,
        KVListType! {(D, Vd), (A, Va), (B, Vb), (C, Vc)},
    >;
    type Assert4 =
        AssertEqual<KVAppendOut<SomeList, D, Vd>, KVListType! {(A, Va), (B, Vb), (C, Vc), (D, Vd)}>;

    type Assert5<Idx> = AssertEqual<
        KVInsertAtOut<SomeList, D, Vd, B, Idx>,
        KVListType! {(A, Va), (B, Vb), (D, Vd), (C, Vc)},
    >;
    type Assert6<Idx> = AssertEqual<
        KVInsertAtOut<SomeList, D, Vd, C, Idx>,
        KVListType! {(A, Va), (B, Vb), (C, Vc), (D, Vd)},
    >;

    type Assert7<Idx> =
        AssertEqual<KVRemoveAtOut<SomeList, B, Idx>, KVListType! {(A, Va), (C, Vc)}>;

    type Assert8<Idx> =
        AssertEqual<KVRemoveManyOut<SomeList, TListType! {A, C}, Idx>, KVListType! {(B, Vb)}>;

    type Assert9<Idx> =
        AssertEqual<KVRemoveManyOut<SomeList, TListType! {B, A, C}, Idx>, KVListType! {}>;

    type Assert10 = AssertEqual<KVReverseOut<SomeList>, KVListType! {(C, Vc), (B, Vb), (A, Va)}>;

    type Assert11<Idx> = KVSetEqualOut<SomeList, KVListType! {(C, Vc), (A, Va), (B, Vb)}, Idx>;

    type Assert12 = AssertEqual<
        KVConcatOut<SomeList, AnotherList>,
        KVListType! {(A, Va), (B, Vb), (C, Vc), (D, Vd), (E, Ve)},
    >;

    #[test]
    fn tlist_test() {
        // prepend empty list
        let _: Assert1 = ();

        // append empty list
        let _: Assert2 = ();

        // prepend non-empty list
        let _: Assert3 = ();

        // append non-empty list
        let _: Assert4 = ();

        // insert in middle
        let _: Assert5<_> = ();

        // insert at end
        let _: Assert6<_> = ();

        // remove
        let _: Assert7<_> = ();

        // remove multiple items
        let _: Assert8<_> = ();

        // remove until empty
        let _: Assert9<_> = ();

        // reverse list
        let _: Assert10 = ();

        // assert identical set of items
        let _: Assert11<_> = ();

        // concat
        let _: Assert12 = ();

        // index of item
        assert_eq!(<SomeList as KVIndexOf<A, _>>::INDEX, 0);
        assert_eq!(<SomeList as KVIndexOf<B, _>>::INDEX, 1);
        assert_eq!(<SomeList as KVIndexOf<C, _>>::INDEX, 2);

        // index of multiple items
        assert_eq!(
            <SomeList as KVIndexOfMany<TListType! {C, A, B}, _>>::indexes(),
            &[2, 0, 1]
        );
    }
}
