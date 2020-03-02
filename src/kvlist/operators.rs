use super::base::{KVCons, KVList};
use crate::{
    counter::{Counter, Step},
    list::{
        self,
        base::{Cons, List, Nil},
    },
};
use std::ops::Sub;
use typenum::{Bit, Sub1, UInt, UTerm, Unsigned, B0, B1};

pub mod ops {
    use super::*;

    // insert at element

    pub trait Insert<Target, Count, Key, Value>
    where
        Self: KVList,
        Self::Output: KVList,
        Count: Counter,
    {
        type Output;
    }

    impl<Target, TargetValue, Key, Value, Tail> Insert<Target, Nil, Key, Value>
        for KVCons<Target, TargetValue, Tail>
    where
        Tail: KVList,
    {
        type Output = KVCons<Key, Value, KVCons<Target, TargetValue, Tail>>;
    }

    impl<Target, Next, Key, Value, CurrKey, CurrValue, Tail> Insert<Target, Step<Next>, Key, Value>
        for KVCons<CurrKey, CurrValue, Tail>
    where
        Next: Counter,
        Tail: KVList + Insert<Target, Next, Key, Value>,
    {
        type Output =
            KVCons<CurrKey, CurrValue, op_aliases::Insert<Tail, Target, Next, Key, Value>>;
    }

    // insert at index

    pub trait InsertAt<Key, Value, Index>
    where
        Self: KVList,
        Self::Output: KVList,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Key, Value, Index, InputList> InsertAt<Key, Value, Index> for InputList
    where
        InputList: KVList + list::ops::InsertAt<(Key, Value), Index>,
        Index: Unsigned,
        list::op_aliases::InsertAt<InputList, (Key, Value), Index>: KVList,
    {
        type Output = list::op_aliases::InsertAt<InputList, (Key, Value), Index>;
    }

    // insert by counter

    pub trait InsertByCounter<Key, Value, Count>
    where
        Self: KVList,
        Self::Output: KVList,
        Count: Counter,
    {
        type Output;
    }

    impl<Key, Value, Count, InputList> InsertByCounter<Key, Value, Count> for InputList
    where
        InputList: KVList + list::ops::InsertByCounter<(Key, Value), Count>,
        Count: Counter,
        list::op_aliases::InsertByCounter<InputList, (Key, Value), Count>: KVList,
    {
        type Output = list::op_aliases::InsertByCounter<InputList, (Key, Value), Count>;
    }

    // remove

    pub trait Remove<Key, Index>
    where
        Self: KVList,
        Self::Output: KVList,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Target, Value, Tail> Remove<Target, UTerm> for KVCons<Target, Value, Tail>
    where
        Tail: KVList,
    {
        type Output = Tail;
    }

    impl<Target, Key, Value, Tail> Remove<Target, UInt<UTerm, B1>> for KVCons<Key, Value, Tail>
    where
        Tail: KVList + Remove<Target, UTerm>,
    {
        type Output = KVCons<Key, Value, op_aliases::Remove<Tail, Target, UTerm>>;
    }

    impl<Target, Key, Value, Tail, U, B> Remove<Target, UInt<UInt<U, B>, B1>>
        for KVCons<Key, Value, Tail>
    where
        Tail: KVList + Remove<Target, UInt<UInt<U, B>, B0>>,
        U: Unsigned,
        B: Bit,
    {
        type Output = KVCons<Key, Value, op_aliases::Remove<Tail, Target, UInt<UInt<U, B>, B0>>>;
    }

    impl<Target, Key, Value, Tail, U> Remove<Target, UInt<U, B0>> for KVCons<Key, Value, Tail>
    where
        Tail: KVList + Remove<Target, UInt<Sub1<U>, B1>>,
        U: Unsigned + Sub<B1>,
        Sub1<U>: Unsigned,
    {
        type Output = KVCons<Key, Value, op_aliases::Remove<Tail, Target, UInt<Sub1<U>, B1>>>;
    }

    // remove by counter

    pub trait RemoveByCounter<Key, Count>
    where
        Self: KVList,
        Self::Output: KVList,
        Count: Counter,
    {
        type Output;
    }

    impl<Target, Value, Tail> RemoveByCounter<Target, Nil> for KVCons<Target, Value, Tail>
    where
        Tail: KVList,
    {
        type Output = Tail;
    }

    impl<Target, Next, Key, Value, Tail> RemoveByCounter<Target, Step<Next>>
        for KVCons<Key, Value, Tail>
    where
        Tail: KVList + RemoveByCounter<Target, Next>,
        Next: Counter,
    {
        type Output = KVCons<Key, Value, op_aliases::RemoveByCounter<Tail, Target, Next>>;
    }

    // keys

    pub trait Keys
    where
        Self: KVList,
        Self::Output: List,
    {
        type Output;
    }

    impl Keys for Nil {
        type Output = Nil;
    }

    impl<Key, Value, Tail> Keys for KVCons<Key, Value, Tail>
    where
        Tail: KVList + Keys,
    {
        type Output = Cons<Key, op_aliases::Keys<Tail>>;
    }

    // values

    pub trait Values
    where
        Self: KVList,
        Self::Output: List,
    {
        type Output;
    }

    impl Values for Nil {
        type Output = Nil;
    }

    impl<Key, Value, Tail> Values for KVCons<Key, Value, Tail>
    where
        Tail: KVList + Values,
    {
        type Output = Cons<Value, op_aliases::Values<Tail>>;
    }

    // index of

    pub trait IndexOf<Key, Index>
    where
        Self: KVList,
        Self::Output: Unsigned,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Key, Index, InputList> IndexOf<Key, Index> for InputList
    where
        Index: Unsigned,
        InputList: KVList + Keys,
        op_aliases::Keys<InputList>: list::ops::IndexOf<Key, Index>,
    {
        type Output = list::op_aliases::IndexOf<op_aliases::Keys<InputList>, Key, Index>;
    }

    // get

    pub trait Get<Target, Count>
    where
        Self: KVList,
        Count: Counter,
    {
        type Output;
    }

    impl<Target, Value, Tail> Get<Target, Nil> for KVCons<Target, Value, Tail>
    where
        Tail: KVList,
    {
        type Output = Value;
    }

    impl<Target, Key, Value, Tail, Next> Get<Target, Step<Next>> for KVCons<Key, Value, Tail>
    where
        Tail: KVList + Get<Target, Next>,
        Next: Counter,
    {
        type Output = op_aliases::Get<Tail, Target, Next>;
    }

    // get key

    pub trait KeyAt<Index>
    where
        Self: KVList,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Index, InputList> KeyAt<Index> for InputList
    where
        Index: Unsigned,
        InputList: KVList + Keys,
        op_aliases::Keys<InputList>: list::ops::At<Index>,
    {
        type Output = list::op_aliases::At<op_aliases::Keys<InputList>, Index>;
    }

    // get value

    pub trait ValueAt<Index>
    where
        Self: KVList,
        Index: Unsigned,
    {
        type Output;
    }

    impl<Index, InputList> ValueAt<Index> for InputList
    where
        Index: Unsigned,
        InputList: KVList + Values,
        op_aliases::Values<InputList>: list::ops::At<Index>,
    {
        type Output = list::op_aliases::At<op_aliases::Values<InputList>, Index>;
    }

    // permute

    pub trait Permute<TargetKeys, Counters>
    where
        Self: KVList,
        Self::Output: KVList,
        TargetKeys: List,
        Counters: List,
    {
        type Output;
    }

    impl Permute<Nil, Nil> for Nil {
        type Output = Nil;
    }

    impl<Target, TargetTail, Count, CounterTail, Head, Tail>
        Permute<Cons<Target, TargetTail>, Cons<Count, CounterTail>> for Cons<Head, Tail>
    where
        Self: Get<Target, Count> + RemoveByCounter<Target, Count>,
        TargetTail: List,
        CounterTail: List,
        Tail: KVList,
        Count: Counter,
        op_aliases::RemoveByCounter<Self, Target, Count>: Permute<TargetTail, CounterTail>,
    {
        type Output = KVCons<
            Target,
            op_aliases::Get<Self, Target, Count>,
            op_aliases::Permute<
                op_aliases::RemoveByCounter<Self, Target, Count>,
                TargetTail,
                CounterTail,
            >,
        >;
    }

}

pub mod op_aliases {
    use super::*;

    pub type Insert<InputList, Target, Count, Key, Value> =
        <InputList as ops::Insert<Target, Count, Key, Value>>::Output;
    pub type InsertAt<InputList, Key, Value, Index> =
        <InputList as ops::InsertAt<Key, Value, Index>>::Output;
    pub type InsertByCounter<InputList, Key, Value, Counter> =
        <InputList as ops::InsertByCounter<Key, Value, Counter>>::Output;
    pub type Remove<InputList, Key, Index> = <InputList as ops::Remove<Key, Index>>::Output;
    pub type RemoveByCounter<InputList, Key, Counter> =
        <InputList as ops::RemoveByCounter<Key, Counter>>::Output;
    pub type Keys<InputList> = <InputList as ops::Keys>::Output;
    pub type Values<InputList> = <InputList as ops::Values>::Output;
    pub type IndexOf<InputList, Key, Index> = <InputList as ops::IndexOf<Key, Index>>::Output;
    pub type KeyAt<InputList, Index> = <InputList as ops::KeyAt<Index>>::Output;
    pub type ValueAt<InputList, Index> = <InputList as ops::ValueAt<Index>>::Output;
    pub type Permute<InputList, TargetKeys, Indexes> =
        <InputList as ops::Permute<TargetKeys, Indexes>>::Output;
    pub type Get<InputList, Target, Counter> = <InputList as ops::Get<Target, Counter>>::Output;

}

#[cfg(test)]
mod tests {
    use super::op_aliases::*;
    use crate::{control::op_aliases::*, KVListT, ListT};
    use typenum::{U0, U1, U2};

    struct Ka;
    struct Kb;
    struct Kc;
    struct Va;
    struct Vb;
    struct Vc;

    type EmptyList = KVListT! {};
    type SingleList = KVListT! {Ka: Va};
    type DoubleList = KVListT! {Ka: Va, Kb: Vb};
    type TripleList = KVListT! {Ka: Va, Kb: Vb, Kc: Vc};

    type Assert1<Index> = AssertSame<InsertAt<EmptyList, Ka, Va, Index>, KVListT! {Ka: Va}, ()>;
    type Assert2 = AssertSame<InsertAt<SingleList, Kb, Vb, U0>, KVListT! {Kb: Vb, Ka: Va}, ()>;
    type Assert3 = AssertSame<InsertAt<SingleList, Kb, Vb, U1>, KVListT! {Ka: Va, Kb: Vb}, ()>;
    type Assert4 =
        AssertSame<InsertAt<DoubleList, Kc, Vc, U1>, KVListT! {Ka: Va, Kc: Vc, Kb: Vb}, ()>;
    type Assert8 =
        AssertSame<InsertAt<DoubleList, Kc, Vc, U2>, KVListT! {Ka: Va, Kb: Vb, Kc: Vc}, ()>;
    type Assert5<Index> = AssertSame<Remove<TripleList, Kb, Index>, KVListT! {Ka: Va, Kc: Vc}, ()>;
    type Assert6<Key> = AssertSame<Remove<TripleList, Key, U2>, KVListT! {Ka: Va, Kb: Vb}, ()>;
    type Assert7 = AssertSame<Keys<TripleList>, ListT![Ka, Kb, Kc], ()>;
    type Assert9 = AssertSame<Values<TripleList>, ListT![Va, Vb, Vc], ()>;
    type Assert10<Index> = AssertSame<IndexOf<TripleList, Kb, Index>, U1, ()>;
    type Assert11 = AssertSame<KeyAt<TripleList, U2>, Kc, ()>;
    type Assert12 = AssertSame<ValueAt<TripleList, U2>, Vc, ()>;
    type Assert13<Counter> = AssertSame<Get<TripleList, Kc, Counter>, Vc, ()>;
    type Assert14<Indexes> =
        AssertSame<Permute<SingleList, ListT![Ka], Indexes>, KVListT! {Ka: Va}, ()>;
    type Assert15<Indexes> =
        AssertSame<Permute<DoubleList, ListT![Kb, Ka], Indexes>, KVListT! {Kb: Vb, Ka: Va}, ()>;
    type Assert16<Indexes> = AssertSame<
        Permute<TripleList, ListT![Kc, Ka, Kb], Indexes>,
        KVListT! {Kc: Vc, Ka: Va, Kb: Vb},
        (),
    >;
    type Assert17<Count> =
        AssertSame<Insert<SingleList, Ka, Count, Kb, Vb>, KVListT! {Kb: Vb, Ka: Va}, ()>;
    type Assert18<Count> =
        AssertSame<Insert<DoubleList, Kb, Count, Kc, Vc>, KVListT! {Ka: Va, Kc: Vc, Kb: Vb}, ()>;

    #[test]
    fn kvlist_ops_test() {
        let _: Assert1<_> = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert8 = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
        let _: Assert7 = ();
        let _: Assert9 = ();
        let _: Assert10<_> = ();
        let _: Assert11 = ();
        let _: Assert12 = ();
        let _: Assert13<_> = ();
        let _: Assert14<_> = ();
        let _: Assert15<_> = ();
        let _: Assert16<_> = ();
        let _: Assert17<_> = ();
        let _: Assert18<_> = ();
    }
}
