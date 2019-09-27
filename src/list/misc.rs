use super::{LCons, LFold, LFoldOutput, LNil, LRemoveAt, LRemoveAtOutput, TList};
use crate::{
    counter::{Counter, Current, Next},
    functional::PrependTListFoldFunc,
};
use std::ops::Add;
use typenum::{Sum, Unsigned, U0, U1};

// length of list

/// A type operator that gets the length of [TList].
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

// set equal

/// Compare if a left-hand-side [TList] has the same set of types
/// with `Rhs` [TList].
///
/// The `Indexes` argument can be left unspecified.
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

// concatenate

/// Concatenates the `Rhs` [TList] to the end of left-hand-side [TList].
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

// split list

pub trait LSplit<Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::FormerOutput: TList,
    Self::LatterOutput: TList,
{
    type FormerOutput;
    type LatterOutput;
}

pub type LSplitFormerOutput<List, Target, Index> = <List as LSplit<Target, Index>>::FormerOutput;
pub type LSplitLatterOutput<List, Target, Index> = <List as LSplit<Target, Index>>::LatterOutput;

impl<Target, Tail> LSplit<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type FormerOutput = LNil;
    type LatterOutput = Self;
}

impl<Target, Index, NonTarget, Tail> LSplit<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LSplit<Target, Index>,
{
    type FormerOutput = LCons<NonTarget, LSplitFormerOutput<Tail, Target, Index>>;
    type LatterOutput = LSplitLatterOutput<Tail, Target, Index>;
}

// reverse

/// Reverses a [TList].
pub trait LReverse
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<List> LReverse for List
where
    List: LFold<LNil, PrependTListFoldFunc>,
    LFoldOutput<List, LNil, PrependTListFoldFunc>: TList,
{
    type Output = LFoldOutput<List, LNil, PrependTListFoldFunc>;
}

pub type LReverseOutput<List> = <List as LReverse>::Output;

// into vector of integers

/// The trait builds a concrete `Vec<usize>` from a [TList]
///
/// It provides [to_usize_vec](LToUsizeVec::to_usize_vec) method to
/// produces a vector of integers. It assumes all contained types implement
/// [Unsigned](typenum::Unsigned) trait.
pub trait LToUsizeVec {
    fn to_usize_vec() -> Vec<usize>;
    fn append_usize_vec(values: &mut Vec<usize>);
}

impl LToUsizeVec for LNil {
    fn to_usize_vec() -> Vec<usize> {
        vec![]
    }

    fn append_usize_vec(_values: &mut Vec<usize>) {}
}

impl<Value, Tail> LToUsizeVec for LCons<Value, Tail>
where
    Value: Unsigned,
    Tail: TList + LToUsizeVec,
{
    fn to_usize_vec() -> Vec<usize> {
        let mut values = vec![];
        Self::append_usize_vec(&mut values);
        values
    }

    fn append_usize_vec(values: &mut Vec<usize>) {
        values.push(Value::USIZE);
        Tail::append_usize_vec(values);
    }
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        control::{IfNotPredicateOutput, IfPredicateOutput, IfSameOutput},
        list::LIndexOfManyIndexes,
        TListType,
    };

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    type SomeList = TListType! {A, B, C};
    type AnotherList = TListType! {D, E};

    // reverse list
    type Assert10 = AssertSame<LReverseOutput<SomeList>, TListType! {C, B, A}>;

    // assert identical set of items
    type Assert11<Idx> = LSetEqualOutput<SomeList, TListType! {C, A, B}, Idx>;

    // concat
    type Assert12 = AssertSame<LConcatOutput<SomeList, AnotherList>, TListType! {A, B, C, D, E}>;

    // index of multiple items
    type Indexes<Idx> = LIndexOfManyIndexes<SomeList, TListType! {C, A, B}, Idx>;

    #[test]
    fn tlist_misc_test() {
        let _: Assert10 = ();
        let _: Assert11<_> = ();
        let _: Assert12 = ();

        assert_eq!(<Indexes<_> as LToUsizeVec>::to_usize_vec(), &[2, 0, 1]);
    }
}
