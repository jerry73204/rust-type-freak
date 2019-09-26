use super::{LCons, LNil, TList};
use crate::counter::{Counter, Current, Next};

// prepend

/// Prepends a new type to [TList].
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

/// Appends a new type to [TList].
pub trait LAppend<Item>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LAppendOutput<List, Item> = <List as LAppend<Item>>::Output;

impl<Item> LAppend<Item> for LNil {
    type Output = LCons<Item, LNil>;
}

impl<Item, Head, Tail> LAppend<Item> for LCons<Head, Tail>
where
    Tail: TList + LAppend<Item>,
{
    type Output = LCons<Head, LAppendOutput<Tail, Item>>;
}

// insert at

/// Inserts a `Item` type to [TList] after `Target` type.
///
/// The trait operator has an auxiliary `Index` argument for
/// list traversal. Usaually it can be left unspecified and
/// the compiler will figure it out.
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
