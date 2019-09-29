use super::{LCons, LNil, TList};
use crate::{
    counter::{Counter, Current, Next},
    functional::{ApplyFunctor, Functor},
};
use std::marker::PhantomData;

// prepend

/// A [Functor] that prepends a new type to [TList].
pub struct LPrependFunctor<Head> {
    _phantom: PhantomData<Head>,
}

pub type LPrepend<List, Item> = ApplyFunctor<LPrependFunctor<Item>, List>;

impl<Item, List> Functor<List> for LPrependFunctor<Item>
where
    List: TList,
{
    type Output = LCons<Item, List>;
}

// prepend to

/// A [Functor] that prepends a new type to [TList].
pub struct LPrependToFunctor<List>
where
    List: TList,
{
    _phantom: PhantomData<List>,
}

pub type LPrependTo<Item, List> = ApplyFunctor<LPrependToFunctor<List>, Item>;

impl<Item, List> Functor<Item> for LPrependToFunctor<List>
where
    List: TList,
{
    type Output = LCons<Item, List>;
}

// prepend accumulator

/// A [Functor] that takes `(List, Item)` input, and prepends `Item` to List of [TList] type.
pub struct LPrependComposeFunctor;

pub type LPrependFold<List, Item> = ApplyFunctor<LPrependComposeFunctor, (List, Item)>;

impl<List, Item> Functor<(List, Item)> for LPrependComposeFunctor
where
    List: TList,
{
    type Output = LCons<Item, List>;
}

// append

/// A type operator that appends a new type to [TList].
pub trait LAppendOp<Item>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LAppendOpOutput<List, Item> = <List as LAppendOp<Item>>::Output;

impl<Item> LAppendOp<Item> for LNil {
    type Output = LCons<Item, LNil>;
}

impl<Item, Head, Tail> LAppendOp<Item> for LCons<Head, Tail>
where
    Tail: TList + LAppendOp<Item>,
{
    type Output = LCons<Head, LAppendOpOutput<Tail, Item>>;
}

/// A [Functor] that appends `Item` to end of [TList].
pub struct LAppendFunctor<Item> {
    _phantom: PhantomData<Item>,
}

pub type LAppend<List, Item> = ApplyFunctor<LAppendFunctor<Item>, List>;

impl<List, Item> Functor<List> for LAppendFunctor<Item>
where
    List: TList + LAppendOp<Item>,
{
    type Output = LAppendOpOutput<List, Item>;
}

// insert at

/// Inserts a `Item` type to [TList] after `Target` type.
///
/// The trait operator has an auxiliary `Index` argument for
/// list traversal. Usaually it can be left unspecified and
/// the compiler will figure it out.
pub trait LInsertAtOp<Item, Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Target, Item, Tail> LInsertAtOp<Item, Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = LCons<Item, LCons<Target, Tail>>;
}

impl<Item, Target, Index, NonTarget, Tail> LInsertAtOp<Item, Target, Next<Index>>
    for LCons<NonTarget, Tail>
where
    Tail: TList + LInsertAtOp<Item, Target, Index>,
    Index: Counter,
{
    type Output = LCons<NonTarget, LInsertAtOpOutput<Tail, Item, Target, Index>>;
}

pub type LInsertAtOpOutput<List, Item, Target, Index> =
    <List as LInsertAtOp<Item, Target, Index>>::Output;

/// A [Functor] that inserts `Item` at `Target` to a [TList].
pub struct LInsertAtFunctor<Item, Target, Index>
where
    Index: Counter,
{
    _phantom: PhantomData<(Item, Target, Index)>,
}

pub type LInsertAt<List, Item, Target, Index> =
    ApplyFunctor<LInsertAtFunctor<Item, Target, Index>, List>;

impl<List, Item, Target, Index> Functor<List> for LInsertAtFunctor<Item, Target, Index>
where
    Index: Counter,
    List: LInsertAtOp<Item, Target, Index>,
{
    type Output = LInsertAtOpOutput<List, Item, Target, Index>;
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

    type EmptyList = TListType![];
    type SomeList = TListType![A, B, C];

    // prepend empty list
    type Assert1 = AssertSame<LPrepend<EmptyList, A>, TListType![A]>;

    // append empty list
    type Assert2 = AssertSame<LAppend<EmptyList, D>, TListType![D]>;

    // prepend non-empty list
    type Assert3 = AssertSame<LPrepend<SomeList, D>, TListType![D, A, B, C]>;

    // append non-empty list
    type Assert4 = AssertSame<LAppend<SomeList, D>, TListType![A, B, C, D]>;

    // insert in middle
    type Assert5<Idx> = AssertSame<LInsertAt<SomeList, D, B, Idx>, TListType![A, D, B, C]>;

    // insert at end
    type Assert6<Idx> = AssertSame<LInsertAt<SomeList, D, C, Idx>, TListType![A, B, D, C]>;

    #[test]
    fn tlist_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
    }
}
