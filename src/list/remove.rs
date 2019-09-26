use super::{LCons, LNil, TList};
use crate::counter::{Counter, Current, Next};

// remove

/// Removes `Target` type from [TList].
///
/// The auxiliary `Index` argument is intended for
/// list traversal. It can be left unspecified and
/// the compiler will figure it out.
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

/// Removes a collection of types from [TList].
///
/// The `Targets` argument accepts a [TList] of types to be removed.
/// The `Indexes` argument can be left unspecified.
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

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, TListType};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;

    type SomeList = TListType! {A, B, C};

    // remove multiple items
    type Assert8<Idx> =
        AssertSame<LRemoveManyOutput<SomeList, TListType! {A, C}, Idx>, TListType! {B}>;

    // remove until empty
    type Assert9<Idx> =
        AssertSame<LRemoveManyOutput<SomeList, TListType! {B, A, C}, Idx>, TListType! {}>;

    #[test]
    fn tlist_test() {
        let _: Assert8<_> = ();
        let _: Assert9<_> = ();
    }
}
