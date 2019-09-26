use super::{LCons, LNil, LRemoveAt, LRemoveAtOutput, TList};
use crate::counter::Counter;
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
