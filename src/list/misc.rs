use super::{
    LCons, LFoldOp, LFoldOpOutput, LNil, LPrependComposeFunctor, LRemoveAtOp, LRemoveAtOpOutput,
    TList,
};
use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
};
use std::marker::PhantomData;
use std::ops::Add;
use typenum::{Sum, Unsigned, U0, U1};

// length of list

/// A type operator that gets the length of [TList].
pub trait LLengthOp
where
    Self: TList,
    Self::Output: Unsigned,
{
    type Output;
}

impl LLengthOp for LNil {
    type Output = U0;
}

impl<Head, Tail> LLengthOp for LCons<Head, Tail>
where
    Tail: TList + LLengthOp,
    LLengthOpOutput<Tail>: Add<U1>,
    Sum<LLengthOpOutput<Tail>, U1>: Unsigned,
{
    type Output = Sum<LLengthOpOutput<Tail>, U1>;
}

pub type LLengthOpOutput<List> = <List as LLengthOp>::Output;

/// A [Functor] that returns the length of [TList].
pub struct LLengthFunctor;

pub type LLength<List> = ApplyFunctor<LLengthFunctor, List>;

impl<List> Functor<List> for LLengthFunctor
where
    List: TList + LLengthOp,
{
    type Output = LLengthOpOutput<List>;
}

// set equal

/// Compare if a left-hand-side [TList] has the same set of types
/// with `Rhs` [TList].
///
/// The `Indexes` argument can be left unspecified.
pub trait LSetEqualOp<Rhs, Indexes>
where
    Rhs: TList,
    Indexes: TList,
    Self: TList,
{
    type Output;
}

pub type LSetEqualOpOutput<Lhs, Rhs, Indexes> = <Lhs as LSetEqualOp<Rhs, Indexes>>::Output;

impl LSetEqualOp<LNil, LNil> for LNil {
    type Output = ();
}

impl<LHead, LTail, RHead, RTail, Index, IRemain>
    LSetEqualOp<LCons<RHead, RTail>, LCons<Index, IRemain>> for LCons<LHead, LTail>
where
    Index: Counter,
    IRemain: TList,
    LTail: TList,
    RTail: TList,
    Self: LRemoveAtOp<RHead, Index>,
    LRemoveAtOpOutput<Self, RHead, Index>: LSetEqualOp<RTail, IRemain>,
{
    type Output = LSetEqualOpOutput<LRemoveAtOpOutput<Self, RHead, Index>, RTail, IRemain>;
}

/// A [Functor] that compares if `Lhs` and `Rhs` [TList]s have same set of values.
pub struct LSetEqualFunctor<Rhs, Indexes> {
    _phantom: PhantomData<(Rhs, Indexes)>,
}

pub type LSetEqual<Lhs, Rhs, Indexes> = ApplyFunctor<LSetEqualFunctor<Rhs, Indexes>, Lhs>;

impl<Lhs, Rhs, Indexes> Functor<Lhs> for LSetEqualFunctor<Rhs, Indexes>
where
    Lhs: TList + LSetEqualOp<Rhs, Indexes>,
    Rhs: TList,
    Indexes: TList,
{
    type Output = LSetEqualOpOutput<Lhs, Rhs, Indexes>;
}

// concatenate

/// Concatenates the `Rhs` [TList] to the end of left-hand-side [TList].
pub trait LConcatOp<Rhs>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Rhs> LConcatOp<Rhs> for LNil
where
    Rhs: TList,
{
    type Output = Rhs;
}

impl<Rhs, Head, Tail> LConcatOp<Rhs> for LCons<Head, Tail>
where
    Rhs: TList,
    Tail: TList + LConcatOp<Rhs>,
{
    type Output = LCons<Head, LConcatOpOutput<Tail, Rhs>>;
}

pub type LConcatOpOutput<Lhs, Rhs> = <Lhs as LConcatOp<Rhs>>::Output;

/// A [Functor] that concatenates input and `Rhs` [TList]s.
pub struct LConcatFunctor<Rhs>
where
    Rhs: TList,
{
    _phantom: PhantomData<Rhs>,
}

pub type LConcat<Lhs, Rhs> = ApplyFunctor<LConcatFunctor<Rhs>, Lhs>;

impl<Lhs, Rhs> Functor<Lhs> for LConcatFunctor<Rhs>
where
    Lhs: TList + LConcatOp<Rhs>,
    Rhs: TList,
{
    type Output = LConcatOpOutput<Lhs, Rhs>;
}

/// A [Functor] that concatenates the input tuple `(Lhs, Rhs)` of [TList]s.
pub struct LConcatComposeFunctor;

pub type LConcatCompose<Lhs, Rhs> = ApplyFunctor<LConcatComposeFunctor, (Lhs, Rhs)>;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for LConcatComposeFunctor
where
    Lhs: TList + LConcatOp<Rhs>,
    Rhs: TList,
{
    type Output = LConcatOpOutput<Lhs, Rhs>;
}

// split list

/// A type operator that splits a [TList] at `Target`.
pub trait LSplitOp<Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::FormerOutput: TList,
    Self::LatterOutput: TList,
{
    type FormerOutput;
    type LatterOutput;
}

pub type LSplitOpFormerOutput<List, Target, Index> =
    <List as LSplitOp<Target, Index>>::FormerOutput;
pub type LSplitOpLatterOutput<List, Target, Index> =
    <List as LSplitOp<Target, Index>>::LatterOutput;

impl<Target, Tail> LSplitOp<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type FormerOutput = LNil;
    type LatterOutput = Self;
}

impl<Target, Index, NonTarget, Tail> LSplitOp<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LSplitOp<Target, Index>,
{
    type FormerOutput = LCons<NonTarget, LSplitOpFormerOutput<Tail, Target, Index>>;
    type LatterOutput = LSplitOpLatterOutput<Tail, Target, Index>;
}

/// A [Functor] that splits input [TList] at `Target`.
pub struct LSplitFunctor<Target, Index>
where
    Index: Counter,
{
    _phantom: PhantomData<(Target, Index)>,
}

pub type LSplit<List, Target, Index> = ApplyFunctor<LSplitFunctor<Target, Index>, List>;

impl<List, Target, Index> Functor<List> for LSplitFunctor<Target, Index>
where
    List: TList + LSplitOp<Target, Index>,
    Index: Counter,
{
    type Output = (
        LSplitOpFormerOutput<List, Target, Index>,
        LSplitOpLatterOutput<List, Target, Index>,
    );
}

// reverse

/// A [Functor] that reverses a [TList].
pub struct LReverseFunctor {}

impl<List> Functor<List> for LReverseFunctor
where
    List: LFoldOp<LNil, LPrependComposeFunctor>,
    LFoldOpOutput<List, LNil, LPrependComposeFunctor>: TList,
{
    type Output = LFoldOpOutput<List, LNil, LPrependComposeFunctor>;
}

pub type LReverse<List> = ApplyFunctor<LReverseFunctor, List>;

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
    use crate::{control::IfSameOutput, TListType};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    type SomeList = TListType![A, B, C];
    type AnotherList = TListType![D, E];

    // split
    type Assert1<Index> = AssertSame<LSplit<SomeList, B, Index>, (TListType![A], TListType![B, C])>;

    // reverse list
    type Assert10 = AssertSame<LReverse<SomeList>, TListType![C, B, A]>;

    // assert identical set of items
    type Assert11<Idx> = LSetEqual<SomeList, TListType![C, A, B], Idx>;

    // concat
    type Assert12 = AssertSame<LConcat<SomeList, AnotherList>, TListType![A, B, C, D, E]>;

    #[test]
    fn tlist_misc_test() {
        let _: Assert1<_> = ();
        let _: Assert10 = ();
        let _: Assert11<_> = ();
        let _: Assert12 = ();
    }
}
