use super::{Cons, List, Nil};
use crate::counter::{Counter, Step};
use std::ops::{Add, Sub};
use typenum::{Add1, Bit, Sub1, UInt, UTerm, Unsigned, B0, B1, U1};

pub mod ops {
    use super::*;

    // append

    pub trait Append<Node>
    where
        Self: List,
        Self::Output: List,
    {
        type Output;
    }

    impl<Node> Append<Node> for Nil {
        type Output = Cons<Node, Nil>;
    }

    impl<Node, Head, Tail> Append<Node> for Cons<Head, Tail>
    where
        Tail: List + Append<Node>,
    {
        type Output = Cons<Head, op_aliases::Append<Tail, Node>>;
    }

    // prepend

    pub trait Prepend<Node>
    where
        Self: List,
        Self::Output: List,
    {
        type Output;
    }

    impl<Node> Prepend<Node> for Nil {
        type Output = Cons<Node, Nil>;
    }

    impl<Node, Head, Tail> Prepend<Node> for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Cons<Node, Self>;
    }

    // insert

    pub trait Insert<Node, Index>
    where
        Index: Unsigned,
        Self: List,
        Self::Output: List,
    {
        type Output;
    }

    impl<Node> Insert<Node, UTerm> for Nil {
        type Output = Cons<Node, Nil>;
    }

    impl<Node, Head, Tail> Insert<Node, UTerm> for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Cons<Node, Self>;
    }

    impl<Node, Head, Tail> Insert<Node, UInt<UTerm, B1>> for Cons<Head, Tail>
    where
        Tail: List + Insert<Node, UTerm>,
    {
        type Output = Cons<Head, op_aliases::Insert<Tail, Node, UTerm>>;
    }

    impl<Node, Head, Tail, U, B> Insert<Node, UInt<UInt<U, B>, B1>> for Cons<Head, Tail>
    where
        Tail: List + Insert<Node, UInt<UInt<U, B>, B0>>,
        U: Unsigned,
        B: Bit,
    {
        type Output = Cons<Head, op_aliases::Insert<Tail, Node, UInt<UInt<U, B>, B0>>>;
    }

    impl<Node, Head, Tail, U> Insert<Node, UInt<U, B0>> for Cons<Head, Tail>
    where
        Tail: List + Insert<Node, UInt<Sub1<U>, B1>>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = Cons<Head, op_aliases::Insert<Tail, Node, UInt<Sub1<U>, B1>>>;
    }

    // insert by counter

    pub trait InsertByCounter<Node, Count>
    where
        Count: Counter,
        Self: List,
        Self::Output: List,
    {
        type Output;
    }

    impl<Node> InsertByCounter<Node, Nil> for Nil {
        type Output = Cons<Node, Nil>;
    }

    impl<Node, Next, Head, Tail> InsertByCounter<Node, Step<Next>> for Cons<Head, Tail>
    where
        Tail: List + InsertByCounter<Node, Next>,
        Next: Counter,
    {
        type Output = Cons<Head, op_aliases::InsertByCounter<Tail, Node, Next>>;
    }

    // remove

    pub trait Remove<Node, Index>
    where
        Self: List,
        Self::Output: List,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Node, Tail> Remove<Node, UTerm> for Cons<Node, Tail>
    where
        Tail: List,
    {
        type Output = Tail;
    }

    impl<Node, Head, Tail> Remove<Node, UInt<UTerm, B1>> for Cons<Head, Tail>
    where
        Tail: List + Remove<Node, UTerm>,
    {
        type Output = Cons<Head, op_aliases::Remove<Tail, Node, UTerm>>;
    }

    impl<Node, Head, Tail, U, B> Remove<Node, UInt<UInt<U, B>, B1>> for Cons<Head, Tail>
    where
        Tail: List + Remove<Node, UInt<UInt<U, B>, B0>>,
        U: Unsigned,
        B: Bit,
    {
        type Output = Cons<Head, op_aliases::Remove<Tail, Node, UInt<UInt<U, B>, B0>>>;
    }

    impl<Node, Head, Tail, U> Remove<Node, UInt<U, B0>> for Cons<Head, Tail>
    where
        Tail: List + Remove<Node, UInt<Sub1<U>, B1>>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = Cons<Head, op_aliases::Remove<Tail, Node, UInt<Sub1<U>, B1>>>;
    }

    // remove by counter

    pub trait RemoveByCounter<Node, Count>
    where
        Self: List,
        Self::Output: List,
        Count: Counter,
    {
        type Output;
    }

    impl<Node, Tail> RemoveByCounter<Node, Nil> for Cons<Node, Tail>
    where
        Tail: List,
    {
        type Output = Tail;
    }

    impl<Node, Next, Head, Tail> RemoveByCounter<Node, Step<Next>> for Cons<Head, Tail>
    where
        Tail: List + RemoveByCounter<Node, Next>,
        Next: Counter,
    {
        type Output = Cons<Head, op_aliases::RemoveByCounter<Tail, Node, Next>>;
    }

    // extend

    pub trait Extend<Other>
    where
        Self: List,
        Self::Output: List,
        Other: List,
    {
        type Output;
    }

    impl<Other> Extend<Other> for Nil
    where
        Other: List,
    {
        type Output = Other;
    }

    impl<Other, Head, Tail> Extend<Other> for Cons<Head, Tail>
    where
        Tail: List + Extend<Other>,
        Other: List,
    {
        type Output = Cons<Head, op_aliases::Extend<Tail, Other>>;
    }

    // at

    pub trait At<Index>
    where
        Self: List,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Head, Tail> At<UTerm> for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Head;
    }

    impl<Head, Tail> At<UInt<UTerm, B1>> for Cons<Head, Tail>
    where
        Tail: List + At<UTerm>,
    {
        type Output = op_aliases::At<Tail, UTerm>;
    }

    impl<Head, Tail, U, B> At<UInt<UInt<U, B>, B1>> for Cons<Head, Tail>
    where
        Tail: List + At<UInt<UInt<U, B>, B0>>,
        U: Unsigned,
        B: Bit,
    {
        type Output = op_aliases::At<Tail, UInt<UInt<U, B>, B0>>;
    }

    impl<Head, Tail, U> At<UInt<U, B0>> for Cons<Head, Tail>
    where
        Tail: List + At<UInt<Sub1<U>, B1>>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = op_aliases::At<Tail, UInt<Sub1<U>, B1>>;
    }

    // at by counter

    pub trait AtByCounter<Index>
    where
        Self: List,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Head, Tail> AtByCounter<UTerm> for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Head;
    }

    impl<Head, Tail> AtByCounter<UInt<UTerm, B1>> for Cons<Head, Tail>
    where
        Tail: List + AtByCounter<UTerm>,
    {
        type Output = op_aliases::AtByCounter<Tail, UTerm>;
    }

    impl<Head, Tail, U, B> AtByCounter<UInt<UInt<U, B>, B1>> for Cons<Head, Tail>
    where
        Tail: List + AtByCounter<UInt<UInt<U, B>, B0>>,
        U: Unsigned,
        B: Bit,
    {
        type Output = op_aliases::AtByCounter<Tail, UInt<UInt<U, B>, B0>>;
    }

    impl<Head, Tail, U> AtByCounter<UInt<U, B0>> for Cons<Head, Tail>
    where
        Tail: List + AtByCounter<UInt<Sub1<U>, B1>>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = op_aliases::AtByCounter<Tail, UInt<Sub1<U>, B1>>;
    }

    // index of

    pub trait IndexOf<Target, Index>
    where
        Self: List,
        Self::Output: Unsigned,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Target, InputList, Index> IndexOf<Target, Index> for InputList
    where
        InputList: List + IndexOfWithBase<Target, UTerm, Index>,
        Index: Unsigned,
    {
        type Output = op_aliases::IndexOfWithBase<InputList, Target, UTerm, Index>;
    }

    // index of op with base

    pub trait IndexOfWithBase<Target, Base, Index>
    where
        Self: List,
        Self::Output: Unsigned,
        Base: Unsigned,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Target, Base, Tail> IndexOfWithBase<Target, Base, UTerm> for Cons<Target, Tail>
    where
        Base: Unsigned,
        Tail: List,
    {
        type Output = Base;
    }

    impl<Target, NonTarget, Base, Tail> IndexOfWithBase<Target, Base, UInt<UTerm, B1>>
        for Cons<NonTarget, Tail>
    where
        Base: Unsigned + Add<B1>,
        Tail: List + IndexOfWithBase<Target, Add1<Base>, UTerm>,
        Add1<Base>: Unsigned,
    {
        type Output = op_aliases::IndexOfWithBase<Tail, Target, Add1<Base>, UTerm>;
    }

    impl<Target, NonTarget, Base, Tail, U, B> IndexOfWithBase<Target, Base, UInt<UInt<U, B>, B1>>
        for Cons<NonTarget, Tail>
    where
        Base: Unsigned + Add<B1>,
        Tail: List + IndexOfWithBase<Target, Add1<Base>, UInt<UInt<U, B>, B0>>,
        Add1<Base>: Unsigned,
        U: Unsigned,
        B: Bit,
    {
        type Output = op_aliases::IndexOfWithBase<Tail, Target, Add1<Base>, UInt<UInt<U, B>, B0>>;
    }

    impl<Target, NonTarget, Base, Tail, U> IndexOfWithBase<Target, Base, UInt<U, B0>>
        for Cons<NonTarget, Tail>
    where
        Base: Unsigned + Add<B1>,
        Tail: List + IndexOfWithBase<Target, Add1<Base>, UInt<Sub1<U>, B1>>,
        Add1<Base>: Unsigned,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = op_aliases::IndexOfWithBase<Tail, Target, Add1<Base>, UInt<Sub1<U>, B1>>;
    }

    // reverse

    pub trait Reverse
    where
        Self: List,
        Self::Output: List,
    {
        type Output;
    }

    impl<InputList> Reverse for InputList
    where
        InputList: List + ReverseWithSuffix<Nil>,
    {
        type Output = op_aliases::ReverseWithSuffix<InputList, Nil>;
    }

    // reverse with suffix

    pub trait ReverseWithSuffix<Suffix>
    where
        Self: List,
        Self::Output: List,
        Suffix: List,
    {
        type Output;
    }

    impl<Suffix> ReverseWithSuffix<Suffix> for Nil
    where
        Suffix: List,
    {
        type Output = Suffix;
    }

    impl<Suffix, Head, Tail> ReverseWithSuffix<Suffix> for Cons<Head, Tail>
    where
        Suffix: List,
        Tail: List + ReverseWithSuffix<Cons<Head, Suffix>>,
    {
        type Output = op_aliases::ReverseWithSuffix<Tail, Cons<Head, Suffix>>;
    }

    // len

    pub trait Len
    where
        Self: List,
        Self::Output: Unsigned,
    {
        type Output;
    }

    impl<InputList> Len for InputList
    where
        InputList: List + LenWithBase<UTerm>,
    {
        type Output = op_aliases::LenWithBase<InputList, UTerm>;
    }

    // len with hbase

    pub trait LenWithBase<Base>
    where
        Self: List,
        Self::Output: Unsigned,
        Base: Unsigned,
    {
        type Output;
    }

    impl<Base> LenWithBase<Base> for Nil
    where
        Base: Unsigned,
    {
        type Output = Base;
    }

    impl<Base, Head, Tail> LenWithBase<Base> for Cons<Head, Tail>
    where
        Tail: List + LenWithBase<Add1<Base>>,
        Base: Unsigned + Add<B1>,
        Add1<Base>: Unsigned,
    {
        type Output = op_aliases::LenWithBase<Tail, Add1<Base>>;
    }

    // first

    pub trait First
    where
        Self: List,
    {
        type Output;
    }

    impl<Head, Tail> First for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Head;
    }

    // last

    pub trait Last
    where
        Self: List,
    {
        type Output;
    }

    impl<Head> Last for Cons<Head, Nil> {
        type Output = Head;
    }

    impl<Head, Next, Tail> Last for Cons<Head, Cons<Next, Tail>>
    where
        Tail: List,
        Cons<Next, Tail>: Last,
    {
        type Output = op_aliases::Last<Cons<Next, Tail>>;
    }

    // replace

    pub trait Replace<Target, Index, New>
    where
        Self: List,
        Self::Output: List,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Target, New, Tail> Replace<Target, UTerm, New> for Cons<Target, Tail>
    where
        Tail: List,
    {
        type Output = Cons<New, Tail>;
    }

    impl<Target, NonTarget, New, Tail> Replace<Target, UInt<UTerm, B1>, New> for Cons<NonTarget, Tail>
    where
        Tail: List + Replace<Target, UTerm, New>,
    {
        type Output = Cons<NonTarget, op_aliases::Replace<Tail, Target, UTerm, New>>;
    }

    impl<Target, NonTarget, New, Tail, U, B> Replace<Target, UInt<UInt<U, B>, B1>, New>
        for Cons<NonTarget, Tail>
    where
        Tail: List + Replace<Target, UInt<UInt<U, B>, B0>, New>,
        U: Unsigned,
        B: Bit,
    {
        type Output = Cons<NonTarget, op_aliases::Replace<Tail, Target, UInt<UInt<U, B>, B0>, New>>;
    }

    impl<Target, NonTarget, New, Tail, U> Replace<Target, UInt<U, B0>, New> for Cons<NonTarget, Tail>
    where
        Tail: List + Replace<Target, UInt<Sub1<U>, B1>, New>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = Cons<NonTarget, op_aliases::Replace<Tail, Target, UInt<Sub1<U>, B1>, New>>;
    }

    // replace by counter

    pub trait ReplaceByCounter<Target, Count, New>
    where
        Self: List,
        Self::Output: List,
        Count: Counter,
    {
        type Output;
    }

    impl<Target, New, Tail> ReplaceByCounter<Target, Nil, New> for Cons<Target, Tail>
    where
        Tail: List,
    {
        type Output = Cons<New, Tail>;
    }

    impl<Target, NonTarget, Next, New, Tail> ReplaceByCounter<Target, Step<Next>, New>
        for Cons<NonTarget, Tail>
    where
        Tail: List + ReplaceByCounter<Target, Next, New>,
        Next: Counter,
    {
        type Output = Cons<NonTarget, op_aliases::ReplaceByCounter<Tail, Target, Next, New>>;
    }

    // range to

    pub trait RangeTo<ToIndex>
    where
        Self: List,
        Self::Output: List,
        ToIndex: Unsigned,
    {
        type Output;
    }

    impl<InputList> RangeTo<UTerm> for InputList
    where
        InputList: List,
    {
        type Output = Nil;
    }

    impl<Head, Tail> RangeTo<UInt<UTerm, B1>> for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Cons<Head, op_aliases::RangeTo<Tail, UTerm>>;
    }

    impl<Head, Tail, U, B> RangeTo<UInt<UInt<U, B>, B1>> for Cons<Head, Tail>
    where
        Tail: List + RangeTo<UInt<UInt<U, B>, B0>>,
        U: Unsigned,
        B: Bit,
    {
        type Output = Cons<Head, op_aliases::RangeTo<Tail, UInt<UInt<U, B>, B0>>>;
    }

    impl<Head, Tail, U> RangeTo<UInt<U, B0>> for Cons<Head, Tail>
    where
        Tail: List + RangeTo<UInt<Sub1<U>, B1>>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = Cons<Head, op_aliases::RangeTo<Tail, UInt<Sub1<U>, B1>>>;
    }

    // range

    pub trait Range<FromIndex, ToIndex>
    where
        Self: List,
        Self::Output: List,
        FromIndex: Unsigned,
        ToIndex: Unsigned,
    {
        type Output;
    }

    impl<ToIndex, Head, Tail> Range<UTerm, ToIndex> for Cons<Head, Tail>
    where
        Self: RangeTo<ToIndex>,
        ToIndex: Unsigned,
        Tail: List,
    {
        type Output = op_aliases::RangeTo<Self, ToIndex>;
    }

    impl<Head, Tail> Range<UInt<UTerm, B1>, UInt<UTerm, B1>> for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Nil;
    }

    impl<Head, Tail, U, B> Range<UInt<UTerm, B1>, UInt<UInt<U, B>, B1>> for Cons<Head, Tail>
    where
        Tail: List + Range<UTerm, UInt<UInt<U, B>, B0>>,
        U: Unsigned,
        B: Bit,
    {
        type Output = op_aliases::Range<Tail, UTerm, UInt<UInt<U, B>, B0>>;
    }

    impl<Head, Tail, U> Range<UInt<UTerm, B1>, UInt<U, B0>> for Cons<Head, Tail>
    where
        Tail: List + Range<UTerm, UInt<Sub1<U>, B1>>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = op_aliases::Range<Tail, UTerm, UInt<Sub1<U>, B1>>;
    }

    impl<Head, Tail, UFrom, BFrom, UTo, BTo>
        Range<UInt<UInt<UFrom, BFrom>, B1>, UInt<UInt<UTo, BTo>, B1>> for Cons<Head, Tail>
    where
        Tail: List + Range<UInt<UInt<UFrom, BFrom>, B0>, UInt<UInt<UTo, BTo>, B0>>,
        UFrom: Unsigned,
        BFrom: Bit,
        UTo: Unsigned,
        BTo: Bit,
    {
        type Output =
            op_aliases::Range<Tail, UInt<UInt<UFrom, BFrom>, B0>, UInt<UInt<UTo, BTo>, B0>>;
    }

    impl<Head, Tail, UFrom, BFrom, UTo> Range<UInt<UInt<UFrom, BFrom>, B1>, UInt<UTo, B0>>
        for Cons<Head, Tail>
    where
        Tail: List + Range<UInt<UInt<UFrom, BFrom>, B0>, UInt<Sub1<UTo>, B1>>,
        UFrom: Unsigned,
        BFrom: Bit,
        UTo: Unsigned + Sub<B1>,
        Sub1<UTo>: Unsigned,
    {
        type Output = op_aliases::Range<Tail, UInt<UInt<UFrom, BFrom>, B0>, UInt<Sub1<UTo>, B1>>;
    }

    impl<Head, Tail, UFrom, UTo, BTo> Range<UInt<UFrom, B0>, UInt<UInt<UTo, BTo>, B1>>
        for Cons<Head, Tail>
    where
        Tail: List + Range<UInt<Sub1<UFrom>, B1>, UInt<UInt<UTo, BTo>, B0>>,
        UFrom: Unsigned + Sub<B1>,
        UTo: Unsigned,
        BTo: Bit,
        Sub1<UFrom>: Unsigned,
    {
        type Output = op_aliases::Range<Tail, UInt<Sub1<UFrom>, B1>, UInt<UInt<UTo, BTo>, B0>>;
    }

    impl<Head, Tail, UFrom, UTo> Range<UInt<UFrom, B0>, UInt<UTo, B0>> for Cons<Head, Tail>
    where
        Tail: List + Range<UInt<Sub1<UFrom>, B1>, UInt<Sub1<UTo>, B1>>,
        UFrom: Unsigned + Sub<B1>,
        UTo: Unsigned + Sub<B1>,
        Sub1<UFrom>: Unsigned,
        Sub1<UTo>: Unsigned,
    {
        type Output = op_aliases::Range<Tail, UInt<Sub1<UFrom>, B1>, UInt<Sub1<UTo>, B1>>;
    }

    // range from

    pub trait RangeFrom<Index>
    where
        Self: List,
        Self::Output: List,
        Index: Unsigned,
    {
        type Output;
    }

    impl<InputList> RangeFrom<UTerm> for InputList
    where
        InputList: List,
    {
        type Output = InputList;
    }

    impl<Head, Tail> RangeFrom<UInt<UTerm, B1>> for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Tail;
    }

    impl<Head, Tail, U, B> RangeFrom<UInt<UInt<U, B>, B1>> for Cons<Head, Tail>
    where
        Tail: List + RangeFrom<UInt<UInt<U, B>, B0>>,
        U: Unsigned,
        B: Bit,
    {
        type Output = op_aliases::RangeFrom<Tail, UInt<UInt<U, B>, B0>>;
    }

    impl<Head, Tail, U> RangeFrom<UInt<U, B0>> for Cons<Head, Tail>
    where
        Tail: List + RangeFrom<UInt<Sub1<U>, B1>>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = op_aliases::RangeFrom<Tail, UInt<Sub1<U>, B1>>;
    }

    // zip

    pub trait Zip<Rhs>
    where
        Self: List,
        Self::Output: List,
        Rhs: List,
    {
        type Output;
    }

    impl Zip<Nil> for Nil {
        type Output = Nil;
    }

    impl<Head, Tail> Zip<Nil> for Cons<Head, Tail>
    where
        Tail: List,
    {
        type Output = Nil;
    }

    impl<Head, Tail> Zip<Cons<Head, Tail>> for Nil
    where
        Tail: List,
    {
        type Output = Nil;
    }

    impl<HeadL, TailL, HeadR, TailR> Zip<Cons<HeadR, TailR>> for Cons<HeadL, TailL>
    where
        TailL: List + Zip<TailR>,
        TailR: List,
    {
        type Output = Cons<(HeadL, HeadR), op_aliases::Zip<TailL, TailR>>;
    }
}

pub mod op_aliases {
    use super::*;

    pub type Append<InputList, Node> = <InputList as ops::Append<Node>>::Output;
    pub type Prepend<InputList, Node> = <InputList as ops::Prepend<Node>>::Output;
    pub type Insert<InputList, Node, Index> = <InputList as ops::Insert<Node, Index>>::Output;
    pub type InsertByCounter<InputList, Node, Counter> =
        <InputList as ops::InsertByCounter<Node, Counter>>::Output;
    pub type Remove<InputList, Node, Index> = <InputList as ops::Remove<Node, Index>>::Output;
    pub type RemoveByCounter<InputList, Node, Counter> =
        <InputList as ops::RemoveByCounter<Node, Counter>>::Output;
    pub type Extend<InputList, OtherList> = <InputList as ops::Extend<OtherList>>::Output;
    pub type At<InputList, Index> = <InputList as ops::At<Index>>::Output;
    pub type AtByCounter<InputList, Counter> = <InputList as ops::AtByCounter<Counter>>::Output;
    pub type IndexOf<InputList, Target, Index> = <InputList as ops::IndexOf<Target, Index>>::Output;
    pub type IndexOfWithBase<InputList, Target, Base, Index> =
        <InputList as ops::IndexOfWithBase<Target, Base, Index>>::Output;
    pub type Reverse<InputList> = <InputList as ops::Reverse>::Output;
    pub type ReverseWithSuffix<InputList, Suffix> =
        <InputList as ops::ReverseWithSuffix<Suffix>>::Output;
    pub type Len<InputList> = <InputList as ops::Len>::Output;
    pub type LenWithBase<InputList, Base> = <InputList as ops::LenWithBase<Base>>::Output;
    pub type First<InputList> = <InputList as ops::First>::Output;
    pub type Last<InputList> = <InputList as ops::Last>::Output;
    pub type Replace<InputList, Target, Index, New> =
        <InputList as ops::Replace<Target, Index, New>>::Output;
    pub type ReplaceByCounter<InputList, Target, Counter, New> =
        <InputList as ops::ReplaceByCounter<Target, Counter, New>>::Output;
    pub type Range<InputList, FromIndex, ToIndex> =
        <InputList as ops::Range<FromIndex, ToIndex>>::Output;
    pub type RangeTo<InputList, ToIndex> = <InputList as ops::RangeTo<ToIndex>>::Output;
    pub type RangeFrom<InputList, FromIndex> = <InputList as ops::RangeFrom<FromIndex>>::Output;
    pub type Zip<Lhs, Rhs> = <Lhs as ops::Zip<Rhs>>::Output;
}

#[cfg(test)]
mod tests {
    use super::op_aliases::*;
    use crate::{control::op_aliases::*, ListT};
    use typenum::{U0, U1, U2, U3};

    struct A;
    struct B;
    struct C;

    type EmptyList = ListT![];
    type SingleList = ListT![A];
    type DoubleList = ListT![B, C];
    type TripleList = ListT![A, B, C];

    type Assert1 = IfSame<(), Append<EmptyList, A>, ListT![A]>;
    type Assert2 = IfSame<(), Append<SingleList, B>, ListT![A, B]>;
    type Assert3 = IfSame<(), Append<DoubleList, A>, ListT![B, C, A]>;
    type Assert4 = IfSame<(), Prepend<EmptyList, A>, ListT![A]>;
    type Assert5 = IfSame<(), Prepend<SingleList, B>, ListT![B, A]>;
    type Assert6 = IfSame<(), Prepend<DoubleList, A>, ListT![A, B, C]>;
    type Assert7<Index> = IfSame<(), Insert<EmptyList, A, Index>, ListT![A]>;
    type Assert8 = IfSame<(), Insert<DoubleList, A, U0>, ListT![A, B, C]>;
    type Assert9 = IfSame<(), Insert<DoubleList, A, U1>, ListT![B, A, C]>;
    type Assert10 = IfSame<(), Insert<DoubleList, A, U2>, ListT![B, C, A]>;
    type Assert11<Index> = IfSame<(), Remove<SingleList, A, Index>, ListT![]>;
    type Assert12<Index> = IfSame<(), Remove<DoubleList, C, Index>, ListT![B]>;
    type Assert13<Element> = IfSame<(), Remove<DoubleList, Element, U0>, ListT![C]>;
    type Assert14 = IfSame<(), Extend<SingleList, DoubleList>, ListT![A, B, C]>;
    type Assert15 = IfSame<(), At<DoubleList, U0>, B>;
    type Assert16 = IfSame<(), At<DoubleList, U1>, C>;
    type Assert17<Index> = IfSame<(), IndexOf<DoubleList, B, Index>, U0>;
    type Assert18<Index> = IfSame<(), IndexOf<DoubleList, C, Index>, U1>;
    type Assert19 = IfSame<(), Reverse<DoubleList>, ListT![C, B]>;
    type Assert20 = IfSame<(), Len<EmptyList>, U0>;
    type Assert21 = IfSame<(), Len<SingleList>, U1>;
    type Assert22 = IfSame<(), Len<DoubleList>, U2>;
    type Assert23 = IfSame<(), First<DoubleList>, B>;
    type Assert24 = IfSame<(), Last<DoubleList>, C>;
    type Assert25<Target> = IfSame<(), Replace<DoubleList, Target, U1, A>, ListT![B, A]>;
    type Assert26<Index> = IfSame<(), Replace<DoubleList, C, Index, A>, ListT![B, A]>;
    type Assert27 = IfSame<(), RangeTo<TripleList, U0>, ListT![]>;
    type Assert28 = IfSame<(), RangeTo<TripleList, U1>, ListT![A]>;
    type Assert29 = IfSame<(), RangeTo<TripleList, U2>, ListT![A, B]>;
    type Assert30 = IfSame<(), RangeTo<TripleList, U3>, ListT![A, B, C]>;
    type Assert31 = IfSame<(), Range<TripleList, U0, U0>, ListT![]>;
    type Assert32 = IfSame<(), Range<TripleList, U0, U1>, ListT![A]>;
    type Assert33 = IfSame<(), Range<TripleList, U0, U2>, ListT![A, B]>;
    type Assert34 = IfSame<(), Range<TripleList, U0, U3>, ListT![A, B, C]>;
    type Assert35 = IfSame<(), Range<TripleList, U1, U1>, ListT![]>;
    type Assert36 = IfSame<(), Range<TripleList, U1, U2>, ListT![B]>;
    type Assert37 = IfSame<(), Range<TripleList, U1, U3>, ListT![B, C]>;
    type Assert38 = IfSame<(), Range<TripleList, U2, U2>, ListT![]>;
    type Assert39 = IfSame<(), Range<TripleList, U2, U3>, ListT![C]>;
    type Assert40 = IfSame<(), Range<TripleList, U3, U3>, ListT![]>;
    type Assert41 = IfSame<(), RangeFrom<TripleList, U0>, ListT![A, B, C]>;
    type Assert42 = IfSame<(), RangeFrom<TripleList, U1>, ListT![B, C]>;
    type Assert43 = IfSame<(), RangeFrom<TripleList, U2>, ListT![C]>;
    type Assert44 = IfSame<(), RangeFrom<TripleList, U3>, ListT![]>;
    type Assert45 = IfSame<(), Zip<DoubleList, TripleList>, ListT![(B, A), (C, B)]>;

    #[test]
    fn list_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7<_> = ();
        let _: Assert8 = ();
        let _: Assert9 = ();
        let _: Assert10 = ();
        let _: Assert11<_> = ();
        let _: Assert12<_> = ();
        let _: Assert13<_> = ();
        let _: Assert14 = ();
        let _: Assert15 = ();
        let _: Assert16 = ();
        let _: Assert17<_> = ();
        let _: Assert18<_> = ();
        let _: Assert19 = ();
        let _: Assert20 = ();
        let _: Assert21 = ();
        let _: Assert22 = ();
        let _: Assert23 = ();
        let _: Assert24 = ();
        let _: Assert25<_> = ();
        let _: Assert26<_> = ();
        let _: Assert27 = ();
        let _: Assert28 = ();
        let _: Assert29 = ();
        let _: Assert30 = ();
        let _: Assert31 = ();
        let _: Assert32 = ();
        let _: Assert33 = ();
        let _: Assert34 = ();
        let _: Assert35 = ();
        let _: Assert36 = ();
        let _: Assert37 = ();
        let _: Assert38 = ();
        let _: Assert39 = ();
        let _: Assert40 = ();
        let _: Assert41 = ();
        let _: Assert42 = ();
        let _: Assert43 = ();
        let _: Assert44 = ();
        let _: Assert45 = ();
    }
}
