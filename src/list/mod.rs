mod reduction;

pub use reduction::*;

use crate::counter::{Counter, Current, Next};
use std::{marker::PhantomData, ops::Add};
use typenum::{Sum, Unsigned, U0, U1};

// list

pub trait TList {}

pub struct LCons<Head, Tail>
where
    Tail: TList,
{
    _phantom: PhantomData<(Head, Tail)>,
}

impl<Head, Tail> LCons<Head, Tail>
where
    Tail: TList,
{
    pub fn new() -> LCons<Head, Tail> {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Head, Tail> TList for LCons<Head, Tail> where Tail: TList {}

pub struct LNil;

impl TList for LNil {}

// length of list

pub trait LLength
where
    Self: TList,
    Self::Out: Unsigned,
{
    type Out;
}

impl LLength for LNil {
    type Out = U0;
}

impl<Head, Tail> LLength for LCons<Head, Tail>
where
    Tail: TList + LLength,
    LLengthOut<Tail>: Add<U1>,
    Sum<LLengthOut<Tail>, U1>: Unsigned,
{
    type Out = Sum<LLengthOut<Tail>, U1>;
}

pub type LLengthOut<List> = <List as LLength>::Out;

// {,non-}empty list trait

pub trait EmptyTList: TList {}

impl EmptyTList for LNil {}

pub trait NonEmptyTList: TList {}

impl<Head, Tail> NonEmptyTList for LCons<Head, Tail> where Tail: TList {}

// prepend

pub trait LPrepend<Head>
where
    Self: TList,
{
    type Out;
}

impl<Item, List> LPrepend<Item> for List
where
    List: TList,
{
    type Out = LCons<Item, List>;
}

pub type LPrependOut<List, Item> = <List as LPrepend<Item>>::Out;

// append

pub trait LAppend<Item>
where
    Self: TList,
    Self::Out: TList,
{
    type Out;
}

impl<Item> LAppend<Item> for LNil {
    type Out = LCons<Item, LNil>;
}

impl<Item, Head, Tail> LAppend<Item> for LCons<Head, Tail>
where
    Tail: TList + LAppend<Item>,
{
    type Out = LCons<Head, LAppendOut<Tail, Item>>;
}

pub type LAppendOut<List, Item> = <List as LAppend<Item>>::Out;

// insert at

pub trait LInsertAt<Item, Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Out: TList,
{
    type Out;
}

impl<Target, Item, Tail> LInsertAt<Item, Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Out = LCons<Target, LCons<Item, Tail>>;
}

impl<Item, Target, Index, NonTarget, Tail> LInsertAt<Item, Target, Next<Index>>
    for LCons<NonTarget, Tail>
where
    Tail: TList + LInsertAt<Item, Target, Index>,
    Index: Counter,
{
    type Out = LCons<NonTarget, LInsertAtOut<Tail, Item, Target, Index>>;
}

pub type LInsertAtOut<List, Item, Target, Index> = <List as LInsertAt<Item, Target, Index>>::Out;

// remove

pub trait LRemoveAt<Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Out: TList,
{
    type Out;
}

impl<Target, Tail> LRemoveAt<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Out = Tail;
}

impl<Target, Index, NonTarget, Tail> LRemoveAt<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LRemoveAt<Target, Index>,
{
    type Out = LCons<NonTarget, LRemoveAtOut<Tail, Target, Index>>;
}

pub type LRemoveAtOut<List, Target, Index> = <List as LRemoveAt<Target, Index>>::Out;

// remove multiple items

pub trait LRemoveMany<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: TList,
    Self::Out: TList,
{
    type Out;
}

impl<List> LRemoveMany<LNil, LNil> for List
where
    List: TList,
{
    type Out = List;
}

impl<Index, IRemain, Target, TRemain, Head, Tail>
    LRemoveMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for LCons<Head, Tail>
where
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Tail: TList,
    Self: LRemoveAt<Target, Index>,
    <Self as LRemoveAt<Target, Index>>::Out: LRemoveMany<TRemain, IRemain>,
{
    type Out = LRemoveManyOut<LRemoveAtOut<Self, Target, Index>, TRemain, IRemain>;
}

pub type LRemoveManyOut<List, Targets, Indexes> = <List as LRemoveMany<Targets, Indexes>>::Out;

// index of item

pub trait LIndexOf<Item, Index>
where
    Self: TList,
    Index: Counter,
{
    const INDEX: usize;
}

impl<Target, Tail> LIndexOf<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    const INDEX: usize = 0;
}

impl<Target, Index, NonTarget, Tail> LIndexOf<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LIndexOf<Target, Index>,
{
    const INDEX: usize = 1 + <Tail as LIndexOf<Target, Index>>::INDEX;
}

// index of many

pub trait LIndexOfMany<Targets, Indexes>
where
    Self: TList,
    Targets: TList,
    Indexes: TList,
{
    fn indexes() -> Vec<usize>;
    fn inverse_indexes() -> Vec<usize>;
}

impl<List> LIndexOfMany<LNil, LNil> for List
where
    List: TList + LLength,
{
    fn indexes() -> Vec<usize> {
        vec![]
    }

    fn inverse_indexes() -> Vec<usize> {
        (0..LLengthOut::<List>::USIZE).collect()
    }
}

impl<Index, IRemain, Target, TRemain, Head, Tail>
    LIndexOfMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for LCons<Head, Tail>
where
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Tail: TList,
    Self: LIndexOf<Target, Index> + LIndexOfMany<TRemain, IRemain>,
{
    fn indexes() -> Vec<usize> {
        let mut indexes = <Self as LIndexOfMany<TRemain, IRemain>>::indexes();
        indexes.insert(0, <Self as LIndexOf<Target, Index>>::INDEX);
        indexes
    }

    fn inverse_indexes() -> Vec<usize> {
        let mut indexes = <Self as LIndexOfMany<TRemain, IRemain>>::inverse_indexes();
        indexes.remove_item(&<Self as LIndexOf<Target, Index>>::INDEX);
        indexes
    }
}

// reverse

pub trait LReverseWithTail<Tail>
where
    Tail: TList,
    Self: TList,
    Self::Out: TList,
{
    type Out;
}

impl<Tail> LReverseWithTail<Tail> for LNil
where
    Tail: TList,
{
    type Out = Tail;
}

impl<ReversedTail, Head, Tail> LReverseWithTail<ReversedTail> for LCons<Head, Tail>
where
    ReversedTail: TList,
    Tail: TList + LReverseWithTail<LCons<Head, ReversedTail>>,
{
    type Out = LReverseWithTailOut<Tail, LCons<Head, ReversedTail>>;
}

pub type LReverseWithTailOut<List, ReversedTail> = <List as LReverseWithTail<ReversedTail>>::Out;
pub type LReverseOut<List> = LReverseWithTailOut<List, LNil>;

// set equal

pub trait LSetEqual<Rhs, Indexes>
where
    Rhs: TList,
    Indexes: TList,
    Self: TList,
{
    type Out;
}

impl LSetEqual<LNil, LNil> for LNil {
    type Out = ();
}

impl<LHead, LTail, RHead, RTail, Index, IRemain>
    LSetEqual<LCons<RHead, RTail>, LCons<Index, IRemain>> for LCons<LHead, LTail>
where
    Index: Counter,
    IRemain: TList,
    LTail: TList,
    RTail: TList,
    Self: LRemoveAt<RHead, Index>,
    LRemoveAtOut<Self, RHead, Index>: LSetEqual<RTail, IRemain>,
{
    type Out = LSetEqualOut<LRemoveAtOut<Self, RHead, Index>, RTail, IRemain>;
}

pub type LSetEqualOut<Lhs, Rhs, Indexes> = <Lhs as LSetEqual<Rhs, Indexes>>::Out;

// combine two identical lists to one

pub trait LCombineEqual<Rhs>
where
    Self: TList,
    Self::Out: TList,
{
    type Out;
}

impl LCombineEqual<LNil> for LNil {
    type Out = LNil;
}

impl<Item, LTail, RTail> LCombineEqual<LCons<Item, RTail>> for LCons<Item, LTail>
where
    LTail: TList + LCombineEqual<RTail>,
    RTail: TList,
    LCombineEqualOut<LTail, RTail>: TList,
{
    type Out = LCons<Item, LCombineEqualOut<LTail, RTail>>;
}

pub type LCombineEqualOut<Lhs, Rhs> = <Lhs as LCombineEqual<Rhs>>::Out;

// concatenate

pub trait LConcat<Rhs>
where
    Self: TList,
    Self::Out: TList,
{
    type Out;
}

impl<Rhs> LConcat<Rhs> for LNil
where
    Rhs: TList,
{
    type Out = Rhs;
}

impl<Rhs, Head, Tail> LConcat<Rhs> for LCons<Head, Tail>
where
    Rhs: TList,
    Tail: TList + LConcat<Rhs>,
{
    type Out = LCons<Head, LConcatOut<Tail, Rhs>>;
}

pub type LConcatOut<Lhs, Rhs> = <Lhs as LConcat<Rhs>>::Out;

// insert if not exist
// TODO test

/// A type operator that inserts a new item if not existing in the list.
pub trait LInsertIfNotExist<Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Out: TList,
{
    type Out;
}

pub type LInsertIfNotExistOut<List, Target, Index> =
    <List as LInsertIfNotExist<Target, Index>>::Out;

impl<Target> LInsertIfNotExist<Target, Current> for LNil {
    type Out = LCons<Target, LNil>;
}

impl<Target, Tail> LInsertIfNotExist<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Out = Self;
}

impl<Target, Index, NonTarget, Tail> LInsertIfNotExist<Target, Next<Index>>
    for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LInsertIfNotExist<Target, Index>,
{
    type Out = LCons<NonTarget, LInsertIfNotExistOut<Tail, Target, Index>>;
}

// functor and fmap for list
// currently not working

// pub trait Functor {
//     type Out<In>;
// }

// pub trait LMap<Func>
// where
//     Self: TList,
//     Func: Functor,
// {
//     type Out;
// }

// pub type LMapOut<List, Func> = <List as LMap<Func>>::Out;

// impl<Func> LMap<Func> for LNil
// where
//     Func: Functor,
// {
//     type Out = LNil;
// }

// impl<Func, Head, Tail> LMap<Func> for LCons<Head, Tail>
// where
//     Func: Functor,
//     Tail: TList + LMap<Func>,
//     LMapOut<Tail, Func>: TList,
// {
//     type Out = LCons<Func::Out<Head>, LMapOut<Tail, Func>>;
// }

// macro

#[macro_export]
macro_rules! TListType {
    () => { $crate::list::LNil };
    ($name:ty) => { $crate::list::LCons<$name, $crate::list::LNil> };
    ($name:ty, $($names:ty),+) => { $crate::list::LCons<$name, $crate::TListType!($($names),*)> };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfOut, TListType};

    type AssertEqual<Lhs, Rhs> = IfOut<(), LCombineEqualOut<Lhs, Rhs>>;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    type EmptyList = TListType! {};
    type SomeList = TListType! {A, B, C};
    type AnotherList = TListType! {D, E};

    type Assert1 = AssertEqual<LPrependOut<EmptyList, A>, TListType! {A}>;
    type Assert2 = AssertEqual<LAppendOut<EmptyList, D>, TListType! {D}>;

    type Assert3 = AssertEqual<LPrependOut<SomeList, D>, TListType! {D, A, B, C}>;
    type Assert4 = AssertEqual<LAppendOut<SomeList, D>, TListType! {A, B, C, D}>;

    type Assert5<Idx> = AssertEqual<LInsertAtOut<SomeList, D, B, Idx>, TListType! {A, B, D, C}>;
    type Assert6<Idx> = AssertEqual<LInsertAtOut<SomeList, D, C, Idx>, TListType! {A, B, C, D}>;

    type Assert7<Idx> = AssertEqual<LRemoveAtOut<SomeList, B, Idx>, TListType! {A, C}>;

    type Assert8<Idx> =
        AssertEqual<LRemoveManyOut<SomeList, TListType! {A, C}, Idx>, TListType! {B}>;

    type Assert9<Idx> =
        AssertEqual<LRemoveManyOut<SomeList, TListType! {B, A, C}, Idx>, TListType! {}>;

    type Assert10 = AssertEqual<LReverseOut<SomeList>, TListType! {C, B, A}>;

    type Assert11<Idx> = LSetEqualOut<SomeList, TListType! {C, A, B}, Idx>;

    type Assert12 = AssertEqual<LConcatOut<SomeList, AnotherList>, TListType! {A, B, C, D, E}>;

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
        assert_eq!(<SomeList as LIndexOf<A, _>>::INDEX, 0);
        assert_eq!(<SomeList as LIndexOf<B, _>>::INDEX, 1);
        assert_eq!(<SomeList as LIndexOf<C, _>>::INDEX, 2);

        // index of multiple items
        assert_eq!(
            <SomeList as LIndexOfMany<TListType! {C, A, B}, _>>::indexes(),
            &[2, 0, 1]
        );
    }
}
