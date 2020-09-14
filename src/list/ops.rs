use super::{Cons, List, Nil};
use crate::{
    common::*,
    functional::Func,
    maybe::{Just, Maybe, Nothing},
    stepper::{Curr, Next, Stepper},
};

typ! {
    pub fn IsEmpty<list>(list: List) -> Bit {
        match list {
            #[generics(head, tail: List)]
            Cons::<head, tail> => false,
            Nil => true,
        }
    }

    pub fn PushFront<list, value>(list: List, value: _) -> List {
        Cons::<value, list>
    }

    pub fn PushBack<list, value>(list: List, value: _) -> List {
        match list {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                let new_tail = PushBack(tail, value);
                Cons::<head, new_tail>
            }
            Nil => {
                Cons::<value, Nil>
            }
        }
    }

    pub fn PopFront<head, tail>(Cons::<head, tail>: List) -> List {
        tail
    }

    pub fn PopBack<head1, tail1: List>(Cons::<head1, tail1>: List) -> List {
        match tail1 {
            #[generics(head2, tail2: List)]
            Cons::<head2, tail2> => {
                let new_tail = PopBack(tail2);
                Cons::<head1, Cons<head2, new_tail>>
            }
            Nil => {
                Nil
            }
        }
    }

    pub fn Insert<list, index, value>(list: List, index: Unsigned, value: _) -> List {
        if index == 0u {
            Cons::<value, list>
        } else {
            match list {
                #[generics(head, tail: List)]
                Cons::<head, tail> => {
                    let new_tail = Insert(tail, index - 1u, value);
                    Cons::<head, new_tail>
                }
            }
        }
    }

    pub fn Remove<head, tail: List, index>(Cons::<head, tail>: List, index: Unsigned) -> List {
        if index == 0u {
            tail
        } else {
            let new_tail = Remove(tail, index - 1u);
            Cons::<head, new_tail>
        }
    }

    pub fn RemoveItem<list, item, step>(list: List, item: _, step: Stepper) -> List {
        match (list, step) {
            #[generics(tail: List)]
            #[capture(item)]
            (Cons::<item, tail>, Curr) => {
                tail
            }
            #[generics(head, tail: List, remaining: Stepper)]
            (Cons::<head, tail>, Next::<remaining>) => {
                let new_tail = RemoveItem(tail, item, remaining);
                Cons::<head, new_tail>
            }
        }
    }

    pub fn ReplaceItem<list, prev, new, step>(list: List, prev: _, new: _, step: Stepper) -> List {
        match (list, step) {
            #[generics(tail: List)]
            #[capture(prev)]
            (Cons::<prev, tail>, Curr) => {
                Cons::<new, tail>
            }
            #[generics(head, tail: List, remaining: Stepper)]
            (Cons::<head, tail>, Next::<remaining>) => {
                let new_tail = ReplaceItem(tail, prev, new, remaining);
                Cons::<head, new_tail>
            }
        }
    }

    pub fn Extend<lhs, rhs>(lhs: List, rhs: List) -> List {
        match lhs {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                let new_tail = Extend(tail, rhs);
                Cons::<head, new_tail>
            }
            Nil => rhs,
        }
    }

    pub fn Get<head, tail: List, index>(Cons::<head, tail>: List, index: Unsigned) {
        if index == 0u {
            head
        } else {
            Get(tail, index - 1u)
        }
    }

    pub fn Reverse<list>(list: List) -> List {
        ReverseRecursive(Nil, list)
    }

    fn ReverseRecursive<saved, remaining>(saved: List, remaining: List) -> List {
        match remaining {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                let new_saved = Cons::<head, saved>;
                ReverseRecursive(new_saved, tail)
            }
            Nil => {
                saved
            }
        }
    }

    pub fn IndexOf<list, item, step>(list: List, item: _, step: Stepper) -> Unsigned {
        match (list, step) {
            #[generics(tail: List)]
            #[capture(item)]
            (Cons::<item, tail>, Curr) => {
                0u
            }
            #[generics(head, tail: List, remaining: Stepper)]
            (Cons::<head, tail>, Next::<remaining>) => {
                IndexOf(tail, item, remaining) + 1u
            }
        }
    }

    pub fn Len<list>(list: List) -> Unsigned {
        match list {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                Len(tail) + 1u
            }
            Nil => 0u,
        }
    }

    pub fn First<head, tail: List>(Cons::<head, tail>: List) {
        head
    }

    pub fn Last<head, tail: List>(Cons::<head, tail>: List) {
        match tail {
            #[generics(head2, tail2: List)]
            Cons::<head2, tail2> => {
                Last(tail)
            }
            Nil => head,
        }
    }

    pub fn Zip<lhs, rhs>(lhs: List, rhs: List) -> List {
        match lhs {
            #[generics(lhead, ltail: List)]
            Cons::<lhead, ltail> => {
                match rhs {
                    #[generics(rhead, rtail: List)]
                    Cons::<rhead, rtail> => {
                        let new_tail = Zip(ltail, rtail);
                        Cons::<(lhead, rhead), new_tail>
                    }
                }
            }
            Nil => {
                match rhs {
                    Nil => Nil
                }
            }
        }
    }

    pub fn Index<list, index>(list: List, index: _) {
        match index {
            UTerm => NumIndex(list, UTerm),
            #[generics(uint: Unsigned, bit: Bit)]
            UInt::<uint, bit> => NumIndex(list, index),
            #[generics(range_index)]
            Range::<range_index> => RangeIndex(list, index),
            #[generics(range_index)]
            RangeInclusive::<range_index> => RangeInclusiveIndex(list, index),
            #[generics(range_index)]
            RangeTo::<range_index> => RangeToIndex(list, index),
            #[generics(range_index)]
            RangeToInclusive::<range_index> => RangeToInclusiveIndex(list, index),
            #[generics(range_index)]
            RangeFrom::<range_index> => RangeFromIndex(list, index),
            RangeFull => list,
        }
    }

    pub fn NumIndex<list, index>(list: List, index: Unsigned) {
        match (list, index) {
            #[generics(head, tail: List)]
            (Cons::<head, tail>, UTerm) => head,
            #[generics(head, tail: List, uint: Unsigned, bit: Bit)]
            (Cons::<head, tail>, UInt::<uint, bit>) => {
                let new_index = index - 1u;
                NumIndex(tail, new_index)
            }
        }
    }

    pub fn RangeIndex<list, range>(list: List, range: _) -> List {
        match (list, range) {
            #[generics(head, tail: List, uint: Unsigned, bit: Bit, upper: Unsigned)]
            (Cons::<head, tail>, Range::<(UInt<uint, bit>, upper)>) => {
                let lower = UInt::<uint, bit>;
                let new_lower = lower - 1u;
                let new_upper = upper - 1u;
                let new_range = Range::<(new_lower, new_upper)>;
                RangeIndex(tail, new_range)
            }
            #[generics(upper: Unsigned)]
            #[capture(list)]
            (list, Range::<(UTerm, upper)>) => {
                let new_range = RangeTo::<upper>;
                RangeToIndex(list, new_range)
            }
        }
    }

    pub fn RangeInclusiveIndex<list, range>(list: List, range: _) -> List {
        match range {
            #[generics(from: Unsigned, to: Unsigned)]
            RangeInclusive::<(from, to)> => {
                let new_to = to + 1u;
                let new_range = Range::<(from, new_to)>;
                RangeIndex(list, new_range)
            }
        }
    }


    pub fn RangeFromIndex<list, range>(list: List, range: _) -> List {
        match (list, range) {
            #[generics(head, tail: List, uint: Unsigned, bit: Bit)]
            (Cons::<head, tail>, RangeFrom::<UInt<uint, bit>>) => {
                let lower = UInt::<uint, bit>;
                let new_lower = lower - 1u;
                let new_range = RangeFrom::<new_lower>;
                RangeFromIndex(tail, new_range)
            }
            #[capture(list)]
            (list, RangeFrom::<UTerm>) => {
                list
            }
        }
    }

    pub fn RangeToIndex<list, range>(list: List, range: _) -> List {
        RangeToIndexRecursive(Nil, list, range)
    }

    pub fn RangeToInclusiveIndex<list, range>(list: List, range: _) -> List {
        match range {
            #[generics(index: Unsigned)]
            RangeToInclusive::<index> => {
                let new_index = index + 1u;
                let new_range = RangeTo::<new_index>;
                RangeToIndexRecursive(Nil, list, new_range)
            }
        }
    }

    fn RangeToIndexRecursive<saved, remaining, range>(saved: List, remaining: List, range: _) -> List {
        match (remaining, range) {
            #[generics(head, tail: List, uint: Unsigned, bit: Bit)]
            (Cons::<head, tail>, RangeTo::<UInt<uint, bit>>) => {
                let new_saved = Cons::<head, saved>;
                let upper = UInt::<uint, bit>;
                let new_upper = upper - 1u;
                let new_range = RangeTo::<new_upper>;
                RangeToIndexRecursive(new_saved, tail, new_range)
            }
            #[capture(remaining)]
            (remaining, RangeTo::<UTerm>) => {
                Reverse(saved)
            }
        }
    }

    pub fn ReduceSum<head1, tail1: List>(Cons::<head1, tail1>: List) {
        match tail1 {
            #[generics(head2, tail2: List)]
            Cons::<head2, tail2> => {
                head1 + ReduceSum(tail1)
            }
            Nil => head1,
        }
    }

    pub fn ReduceProduct<head1, tail1: List>(Cons::<head1, tail1>: List) {
        match tail1 {
            #[generics(head2, tail2: List)]
            Cons::<head2, tail2> => {
                head1 * ReduceProduct(tail1)
            }
            Nil => head1,
        }
    }

    pub fn ReduceMax<head1, tail1: List>(Cons::<head1, tail1>: List) {
        match tail1 {
            #[generics(head2, tail2: List)]
            Cons::<head2, tail2> => {
                let tail_max = ReduceMax(tail1);
                <head1 as Max<tail_max>>::Output
            }
            Nil => head1,
        }
    }

    pub fn ReduceMin<head1, tail1: List>(Cons::<head1, tail1>: List) {
        match tail1 {
            #[generics(head2, tail2: List)]
            Cons::<head2, tail2> => {
                let tail_max = ReduceMax(tail1);
                <head1 as Min<tail_max>>::Output
            }
            Nil => head1,
        }
    }

    pub fn All<list>(list: List) -> Bit {
        match list {
            #[generics(head: Bit, tail: List)]
            Cons::<head, tail> => {
                if head {
                    All(tail)
                } else {
                    false
                }
            }
            Nil => true,
        }
    }

    pub fn Any<list>(list: List) -> Bit {
        match list {
            #[generics(head: Bit, tail: List)]
            Cons::<head, tail> => {
                if head {
                    true
                } else {
                    Any(tail)
                }
            }
            Nil => false,
        }
    }

    pub fn Map<list, func>(list: List, func: _) -> List {
        match list {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                let new_head = <func as Func<head>>::Output;
                let new_tail = Map(tail, func);
                Cons::<new_head, new_tail>
            }
            Nil => Nil,
        }
    }

    pub fn Filter<list, func>(list: List, func: _) -> List {
        match list {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                let keep: Bit = <func as Func<head>>::Output;
                let new_tail = Filter(tail, func);
                if keep {
                    Cons::<head, new_tail>
                } else {
                    new_tail
                }
            }
            Nil => Nil,
        }
    }

    pub fn FilterMap<list, func>(list: List, func: _) -> List {
        match list {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                let maybe: Maybe = <func as Func<head>>::Output;
                let new_tail = Filter(tail, func);

                match maybe {
                    #[generics(new_head)]
                    Just::<new_head> => Cons::<new_head, new_tail>,
                    Nothing => new_tail
                }
            }
            Nil => Nil,
        }
    }

    pub fn Fold<list, init, func>(list: List, init: _, func: _) {
        match list {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                let new_init = <func as Func<(init, head)>>::Output;
                Fold(tail, new_init, func)
            }
            Nil => init,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::op_aliases::*, List};
    use typenum::consts::*;

    struct A;
    struct B;
    struct C;
    struct D;

    type Assert1 = AssertSame<PushBackOp<List![], A>, List![A], ()>;
    type Assert2 = AssertSame<PushBackOp<List![A], B>, List![A, B], ()>;
    type Assert3 = AssertSame<PushBackOp<List![B, C], A>, List![B, C, A], ()>;
    type Assert4 = AssertSame<PushFrontOp<List![], A>, List![A], ()>;
    type Assert5 = AssertSame<PushFrontOp<List![A], B>, List![B, A], ()>;
    type Assert6 = AssertSame<PushFrontOp<List![B, C], A>, List![A, B, C], ()>;
    type Assert7 = AssertSame<InsertOp<List![], U0, A>, List![A], ()>;
    type Assert8 = AssertSame<InsertOp<List![B, C], U0, A>, List![A, B, C], ()>;
    type Assert9 = AssertSame<InsertOp<List![B, C], U1, A>, List![B, A, C], ()>;
    type Assert10 = AssertSame<InsertOp<List![B, C], U2, A>, List![B, C, A], ()>;
    type Assert11<Index> = AssertSame<RemoveItemOp<List![A], A, Index>, List![], ()>;
    type Assert12<Index> = AssertSame<RemoveItemOp<List![B, C], C, Index>, List![B], ()>;
    type Assert14 = AssertSame<ExtendOp<List![A], List![B, C]>, List![A, B, C], ()>;
    type Assert15 = AssertSame<GetOp<List![B, C], U0>, B, ()>;
    type Assert16 = AssertSame<GetOp<List![B, C], U1>, C, ()>;
    type Assert17<Index> = AssertSame<IndexOfOp<List![B, C], B, Index>, U0, ()>;
    type Assert18<Index> = AssertSame<IndexOfOp<List![B, C], C, Index>, U1, ()>;
    type Assert19 = AssertSame<ReverseOp<List![B, C]>, List![C, B], ()>;
    type Assert20 = AssertSame<LenOp<List![]>, U0, ()>;
    type Assert21 = AssertSame<LenOp<List![A]>, U1, ()>;
    type Assert22 = AssertSame<LenOp<List![B, C]>, U2, ()>;
    type Assert23 = AssertSame<FirstOp<List![B, C]>, B, ()>;
    type Assert24 = AssertSame<LastOp<List![B, C]>, C, ()>;
    type Assert25<Stepper> = AssertSame<ReplaceItemOp<List![B, C], B, D, Stepper>, List![D, C], ()>;
    type Assert45 = AssertSame<ZipOp<List![A, B], List![C, D]>, List![(A, C), (B, D)], ()>;
    type Assert48 = AssertSame<PopFrontOp<List![A, B, C]>, List![B, C], ()>;
    type Assert49 = AssertSame<PopBackOp<List![A, B, C]>, List![A, B], ()>;

    #[test]
    fn list_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
        let _: Assert9 = ();
        let _: Assert10 = ();
        let _: Assert11<_> = ();
        let _: Assert12<_> = ();
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
        let _: Assert45 = ();
        let _: Assert48 = ();
        let _: Assert49 = ();
        let _: AssertSame<ReverseOp<List![A, B, C]>, List![C, B, A], ()> = ();
        let _: AssertSame<IsEmptyOp<List![]>, B1, ()> = ();
        let _: AssertSame<IsEmptyOp<List![A]>, B0, ()> = ();
        let _: AssertSame<IsEmptyOp<List![A, B, C]>, B0, ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], U0>, A, ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], U1>, B, ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], U2>, C, ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeFull>, List![A, B, C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], Range<(U0, U0)>>, List![], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], Range<(U1, U1)>>, List![], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], Range<(U2, U2)>>, List![], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], Range<(U0, U1)>>, List![A], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], Range<(U2, U3)>>, List![C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], Range<(U1, U3)>>, List![B, C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeInclusive<(U0, U0)>>, List![A], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeInclusive<(U1, U1)>>, List![B], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeInclusive<(U2, U2)>>, List![C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeInclusive<(U0, U1)>>, List![A, B], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeInclusive<(U0, U2)>>, List![A, B, C], ()> =
            ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeInclusive<(U1, U2)>>, List![B, C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeTo<U0>>, List![], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeTo<U1>>, List![A], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeTo<U2>>, List![A, B], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeTo<U3>>, List![A, B, C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeToInclusive<U0>>, List![A], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeToInclusive<U1>>, List![A, B], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeToInclusive<U2>>, List![A, B, C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeFrom<U0>>, List![A, B, C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeFrom<U1>>, List![B, C], ()> = ();
        let _: AssertSame<IndexOp<List![A, B, C], RangeFrom<U2>>, List![C], ()> = ();
        let _: AssertSame<ReduceSumOp<List![U3, U2, U7]>, U12, ()> = ();
        let _: AssertSame<ReduceProductOp<List![U3, U2, U7]>, U42, ()> = ();
    }
}
