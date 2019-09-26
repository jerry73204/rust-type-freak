use super::{LCons, LNil, TList};

// reverse with a tail to append

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

// reverse

pub trait LReverse
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<List> LReverse for List
where
    List: TList + LReverseWithTail<LNil>,
{
    type Output = LReverseWithTailOutput<List, LNil>;
}

pub type LReverseOutput<List> = <List as LReverse>::Output;

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

    // reverse list
    type Assert10 = AssertSame<LReverseOutput<SomeList>, TListType! {C, B, A}>;

    #[test]
    fn tlist_test() {
        let _: Assert10 = ();
    }
}
