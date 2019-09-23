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
    Self::Output: Unsigned,
{
    type Output;
}

impl LLength for LNil {
    type Output = U0;
}

impl<Head, Tail> LLength for LCons<Head, Tail>
where
    Tail: TList + LLength,
    LLengthOutput<Tail>: Add<U1>,
    Sum<LLengthOutput<Tail>, U1>: Unsigned,
{
    type Output = Sum<LLengthOutput<Tail>, U1>;
}

pub type LLengthOutput<List> = <List as LLength>::Output;

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
    type Output;
}

impl<Item, List> LPrepend<Item> for List
where
    List: TList,
{
    type Output = LCons<Item, List>;
}

pub type LPrependOutput<List, Item> = <List as LPrepend<Item>>::Output;

// append

pub trait LAppend<Item>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Item> LAppend<Item> for LNil {
    type Output = LCons<Item, LNil>;
}

impl<Item, Head, Tail> LAppend<Item> for LCons<Head, Tail>
where
    Tail: TList + LAppend<Item>,
{
    type Output = LCons<Head, LAppendOutput<Tail, Item>>;
}

pub type LAppendOutput<List, Item> = <List as LAppend<Item>>::Output;

// insert at

pub trait LInsertAt<Item, Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Target, Item, Tail> LInsertAt<Item, Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = LCons<Target, LCons<Item, Tail>>;
}

impl<Item, Target, Index, NonTarget, Tail> LInsertAt<Item, Target, Next<Index>>
    for LCons<NonTarget, Tail>
where
    Tail: TList + LInsertAt<Item, Target, Index>,
    Index: Counter,
{
    type Output = LCons<NonTarget, LInsertAtOutput<Tail, Item, Target, Index>>;
}

pub type LInsertAtOutput<List, Item, Target, Index> =
    <List as LInsertAt<Item, Target, Index>>::Output;

// remove

pub trait LRemoveAt<Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Target, Tail> LRemoveAt<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = Tail;
}

impl<Target, Index, NonTarget, Tail> LRemoveAt<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LRemoveAt<Target, Index>,
{
    type Output = LCons<NonTarget, LRemoveAtOutput<Tail, Target, Index>>;
}

pub type LRemoveAtOutput<List, Target, Index> = <List as LRemoveAt<Target, Index>>::Output;

// remove multiple items

pub trait LRemoveMany<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<List> LRemoveMany<LNil, LNil> for List
where
    List: TList,
{
    type Output = List;
}

impl<Index, IRemain, Target, TRemain, Head, Tail>
    LRemoveMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for LCons<Head, Tail>
where
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Tail: TList,
    Self: LRemoveAt<Target, Index>,
    <Self as LRemoveAt<Target, Index>>::Output: LRemoveMany<TRemain, IRemain>,
{
    type Output = LRemoveManyOutput<LRemoveAtOutput<Self, Target, Index>, TRemain, IRemain>;
}

pub type LRemoveManyOutput<List, Targets, Indexes> =
    <List as LRemoveMany<Targets, Indexes>>::Output;

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
        (0..LLengthOutput::<List>::USIZE).collect()
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
    Self::Output: TList,
{
    type Output;
}

impl<Tail> LReverseWithTail<Tail> for LNil
where
    Tail: TList,
{
    type Output = Tail;
}

impl<ReversedTail, Head, Tail> LReverseWithTail<ReversedTail> for LCons<Head, Tail>
where
    ReversedTail: TList,
    Tail: TList + LReverseWithTail<LCons<Head, ReversedTail>>,
{
    type Output = LReverseWithTailOutput<Tail, LCons<Head, ReversedTail>>;
}

pub type LReverseWithTailOutput<List, ReversedTail> =
    <List as LReverseWithTail<ReversedTail>>::Output;
pub type LReverseOutput<List> = LReverseWithTailOutput<List, LNil>;

// set equal

pub trait LSetEqual<Rhs, Indexes>
where
    Rhs: TList,
    Indexes: TList,
    Self: TList,
{
    type Output;
}

impl LSetEqual<LNil, LNil> for LNil {
    type Output = ();
}

impl<LHead, LTail, RHead, RTail, Index, IRemain>
    LSetEqual<LCons<RHead, RTail>, LCons<Index, IRemain>> for LCons<LHead, LTail>
where
    Index: Counter,
    IRemain: TList,
    LTail: TList,
    RTail: TList,
    Self: LRemoveAt<RHead, Index>,
    LRemoveAtOutput<Self, RHead, Index>: LSetEqual<RTail, IRemain>,
{
    type Output = LSetEqualOutput<LRemoveAtOutput<Self, RHead, Index>, RTail, IRemain>;
}

pub type LSetEqualOutput<Lhs, Rhs, Indexes> = <Lhs as LSetEqual<Rhs, Indexes>>::Output;

// combine two identical lists to one

pub trait LCombineEqual<Rhs>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl LCombineEqual<LNil> for LNil {
    type Output = LNil;
}

impl<Item, LTail, RTail> LCombineEqual<LCons<Item, RTail>> for LCons<Item, LTail>
where
    LTail: TList + LCombineEqual<RTail>,
    RTail: TList,
    LCombineEqualOutput<LTail, RTail>: TList,
{
    type Output = LCons<Item, LCombineEqualOutput<LTail, RTail>>;
}

pub type LCombineEqualOutput<Lhs, Rhs> = <Lhs as LCombineEqual<Rhs>>::Output;

// concatenate

pub trait LConcat<Rhs>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Rhs> LConcat<Rhs> for LNil
where
    Rhs: TList,
{
    type Output = Rhs;
}

impl<Rhs, Head, Tail> LConcat<Rhs> for LCons<Head, Tail>
where
    Rhs: TList,
    Tail: TList + LConcat<Rhs>,
{
    type Output = LCons<Head, LConcatOutput<Tail, Rhs>>;
}

pub type LConcatOutput<Lhs, Rhs> = <Lhs as LConcat<Rhs>>::Output;

// insert if not exist
// TODO test

/// A type operator that inserts a new item if not existing in the list.
pub trait LInsertIfNotExist<Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LInsertIfNotExistOutput<List, Target, Index> =
    <List as LInsertIfNotExist<Target, Index>>::Output;

impl<Target> LInsertIfNotExist<Target, Current> for LNil {
    type Output = LCons<Target, LNil>;
}

impl<Target, Tail> LInsertIfNotExist<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = Self;
}

impl<Target, Index, NonTarget, Tail> LInsertIfNotExist<Target, Next<Index>>
    for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LInsertIfNotExist<Target, Index>,
{
    type Output = LCons<NonTarget, LInsertIfNotExistOutput<Tail, Target, Index>>;
}

// functor and fmap for list
// currently not working

// pub trait Functor {
//     type Output<In>;
// }

// pub trait LMap<Func>
// where
//     Self: TList,
//     Func: Functor,
// {
//     type Output;
// }

// pub type LMapOutput<List, Func> = <List as LMap<Func>>::Output;

// impl<Func> LMap<Func> for LNil
// where
//     Func: Functor,
// {
//     type Output = LNil;
// }

// impl<Func, Head, Tail> LMap<Func> for LCons<Head, Tail>
// where
//     Func: Functor,
//     Tail: TList + LMap<Func>,
//     LMapOutput<Tail, Func>: TList,
// {
//     type Output = LCons<Func::Output<Head>, LMapOutput<Tail, Func>>;
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
    use crate::{control::IfOutput, TListType};

    type AssertEqual<Lhs, Rhs> = IfOutput<(), LCombineEqualOutput<Lhs, Rhs>>;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    type EmptyList = TListType! {};
    type SomeList = TListType! {A, B, C};
    type AnotherList = TListType! {D, E};

    type Assert1 = AssertEqual<LPrependOutput<EmptyList, A>, TListType! {A}>;
    type Assert2 = AssertEqual<LAppendOutput<EmptyList, D>, TListType! {D}>;

    type Assert3 = AssertEqual<LPrependOutput<SomeList, D>, TListType! {D, A, B, C}>;
    type Assert4 = AssertEqual<LAppendOutput<SomeList, D>, TListType! {A, B, C, D}>;

    type Assert5<Idx> = AssertEqual<LInsertAtOutput<SomeList, D, B, Idx>, TListType! {A, B, D, C}>;
    type Assert6<Idx> = AssertEqual<LInsertAtOutput<SomeList, D, C, Idx>, TListType! {A, B, C, D}>;

    type Assert7<Idx> = AssertEqual<LRemoveAtOutput<SomeList, B, Idx>, TListType! {A, C}>;

    type Assert8<Idx> =
        AssertEqual<LRemoveManyOutput<SomeList, TListType! {A, C}, Idx>, TListType! {B}>;

    type Assert9<Idx> =
        AssertEqual<LRemoveManyOutput<SomeList, TListType! {B, A, C}, Idx>, TListType! {}>;

    type Assert10 = AssertEqual<LReverseOutput<SomeList>, TListType! {C, B, A}>;

    type Assert11<Idx> = LSetEqualOutput<SomeList, TListType! {C, A, B}, Idx>;

    type Assert12 = AssertEqual<LConcatOutput<SomeList, AnotherList>, TListType! {A, B, C, D, E}>;

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
