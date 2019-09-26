use super::{LCons, LNil, NonEmptyTList, TList, TListType};
use crate::counter::{Counter, Current, Next};
use std::ops::Add;
use typenum::{Add1, Unsigned, B1, U0};

// index of item

/// Returns the position of `Target` type in [TList].
///
/// The returned outcome always implements [Unsigned](typenum::Unsigned)
/// trait. The `Index` argument can be left unspecified.
pub trait LIndexOf<Target, Index>
where
    Self: TList,
    Index: Counter,
    Self::Index: Unsigned,
{
    type Index;
}

pub type LIndexOfIndex<List, Target, Index> = <List as LIndexOf<Target, Index>>::Index;

impl<Target, Tail> LIndexOf<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Index = U0;
}

impl<Target, Index, NonTarget, Tail> LIndexOf<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LIndexOf<Target, Index>,
    LIndexOfIndex<Tail, Target, Index>: Add<B1>,
    Add1<LIndexOfIndex<Tail, Target, Index>>: Unsigned,
{
    type Index = Add1<LIndexOfIndex<Tail, Target, Index>>;
}

// index of many

/// Gets indexes of multiple types from [TList].
///
/// The `Targets` argument is a [TList] of queried types.
/// The `Indexes` can be left unspecified.
pub trait LIndexOfMany<Targets, Indexes>
where
    Self: TList,
    Targets: TList,
    Indexes: TList,
    Self::Indexes: TList,
{
    type Indexes;
}

pub type LIndexOfManyIndexes<List, Targets, Indexes> =
    <List as LIndexOfMany<Targets, Indexes>>::Indexes;

impl<List> LIndexOfMany<LNil, LNil> for List
where
    List: TList,
{
    type Indexes = TListType! {};
}

impl<List, Index, IRemain, Target, TRemain>
    LIndexOfMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for List
where
    List: NonEmptyTList,
    Index: Counter,
    IRemain: TList,
    TRemain: TList,
    Self: LIndexOfMany<TRemain, IRemain> + LIndexOf<Target, Index>,
{
    type Indexes =
        LCons<LIndexOfIndex<Self, Target, Index>, LIndexOfManyIndexes<Self, TRemain, IRemain>>;
}
