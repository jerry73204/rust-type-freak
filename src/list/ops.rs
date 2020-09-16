use super::{Cons, List, Nil};
use crate::{
    common::*,
    functional::Func,
    maybe::{Just, Maybe, Nothing},
    stepper::{Curr, Next, Stepper},
    tuple::{Get0, Get1, Tuple2},
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
                    let new_index: Unsigned = index - 1u;
                    let new_tail = Insert(tail, new_index, value);
                    Cons::<head, new_tail>
                }
            }
        }
    }

    pub fn Remove<head, tail: List, index>(Cons::<head, tail>: List, index: Unsigned) -> List {
        if index == 0u {
            tail
        } else {
            let new_index: Unsigned = index - 1u;
            let new_tail = Remove(tail, new_index);
            Cons::<head, new_tail>
        }
    }

    pub fn RemoveItem<list, item, step>(list: List, item: _, step: Stepper) -> List {
        match (list, step) {
            #[generics(tail: List)]
            #[capture(item)]
            (Cons::<item, tail>, Curr) => tail,
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
            let new_index: Unsigned = index - 1u;
            Get(tail, new_index)
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
            (Cons::<item, tail>, Curr) => 0u,
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
            UInt::<uint, bit> => {
                let index: Unsigned = index;
                NumIndex(list, index)
            }
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
                let new_index: Unsigned = index - 1u;
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
                head1.Max(tail_max)
            }
            Nil => head1,
        }
    }

    pub fn ReduceMin<head1, tail1: List>(Cons::<head1, tail1>: List) {
        match tail1 {
            #[generics(head2, tail2: List)]
            Cons::<head2, tail2> => {
                let tail_min = ReduceMax(tail1);
                head1.Min(tail_min)
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
                let new_head = func.Func(head);
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
                let keep: Bit = func.Func(head);
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
                let maybe: Maybe = func.Func(head);
                let new_tail = FilterMap(tail, func);

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
                let new_init = func.Func((init, head));
                Fold(tail, new_init, func)
            }
            Nil => init,
        }
    }

    pub fn Scan<list, state, func>(list: List, state: _, func: _) -> List {
        match list {
            #[generics(item, tail: List)]
            Cons::<item, tail> => {
                let tuple: Tuple2 = func.Func((state, item));
                let new_state = tuple.Get0();
                let new_item = tuple.Get1();
                let new_tail = Scan(tail, new_state, func);
                Cons::<new_item, new_tail>
            }
            Nil => Nil,
        }
    }
}

typ! {
    pub fn ZipEx<lists>(lists: List) -> List {
        Recursive(Nil, lists)
    }

    fn Recursive<saved, remaining>(saved: List, remaining: List) -> List {
        match remaining {
            #[generics(item, tail1: List, tail2: List)]
            Cons::<Cons::<item, tail1>, tail2> => {
                let tuple = Step(Nil, Nil, remaining);
                let zipped: List = tuple.Get0();
                let new_remaining: List = tuple.Get1();
                let new_saved = Cons::<zipped, saved>;
                Recursive(new_saved, new_remaining)
           }
            #[generics(tail: List)]
            Cons::<Nil, tail> => {
                AssertEnd(remaining);
                Reverse(saved)
            }
            Nil => Nil,
        }
    }

    fn Step<zipped, saved, remaining>(zipped: List, saved: List, remaining: List) -> Tuple2 {
        match remaining {
            #[generics(item, tail1: List, tail2: List)]
            Cons::<Cons::<item, tail1>, tail2> => {
                let new_zipped = Cons::<item, zipped>;
                let new_saved = Cons::<tail1, saved>;
                let new_remaining = tail2;
                Step(new_zipped, new_saved, new_remaining)
            }
            Nil => (Reverse(zipped), Reverse(saved)),
        }
    }

    fn AssertEnd<remaining>(remaining: List) {
        match remaining {
            #[generics(tail: List)]
            Cons::<Nil, tail> => AssertEnd(tail),
            Nil => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        control::SameOp,
        maybe::{Just, Maybe, Nothing},
        List,
    };
    use typenum::consts::*;

    struct A;
    struct B;
    struct C;
    struct D;

    #[test]
    fn list_test() {
        let _: SameOp<PushBackOp<List![], A>, List![A]> = ();
        let _: SameOp<PushBackOp<List![A], B>, List![A, B]> = ();
        let _: SameOp<PushBackOp<List![B, C], A>, List![B, C, A]> = ();
        let _: SameOp<PushFrontOp<List![], A>, List![A]> = ();
        let _: SameOp<PushFrontOp<List![A], B>, List![B, A]> = ();
        let _: SameOp<PushFrontOp<List![B, C], A>, List![A, B, C]> = ();
        let _: SameOp<InsertOp<List![], U0, A>, List![A]> = ();
        let _: SameOp<InsertOp<List![B, C], U0, A>, List![A, B, C]> = ();
        let _: SameOp<InsertOp<List![B, C], U1, A>, List![B, A, C]> = ();
        let _: SameOp<InsertOp<List![B, C], U2, A>, List![B, C, A]> = ();
        let _: SameOp<RemoveItemOp<List![A], A, _>, List![]> = ();
        let _: SameOp<RemoveItemOp<List![B, C], C, _>, List![B]> = ();
        let _: SameOp<ExtendOp<List![A], List![B, C]>, List![A, B, C]> = ();
        let _: SameOp<GetOp<List![B, C], U0>, B> = ();
        let _: SameOp<GetOp<List![B, C], U1>, C> = ();
        let _: SameOp<IndexOfOp<List![B, C], B, _>, U0> = ();
        let _: SameOp<IndexOfOp<List![B, C], C, _>, U1> = ();
        let _: SameOp<ReverseOp<List![B, C]>, List![C, B]> = ();
        let _: SameOp<LenOp<List![]>, U0> = ();
        let _: SameOp<LenOp<List![A]>, U1> = ();
        let _: SameOp<LenOp<List![B, C]>, U2> = ();
        let _: SameOp<FirstOp<List![B, C]>, B> = ();
        let _: SameOp<LastOp<List![B, C]>, C> = ();
        let _: SameOp<ReplaceItemOp<List![B, C], B, D, _>, List![D, C]> = ();
        let _: SameOp<ZipOp<List![A, B], List![C, D]>, List![(A, C), (B, D)]> = ();
        let _: SameOp<PopFrontOp<List![A, B, C]>, List![B, C]> = ();
        let _: SameOp<PopBackOp<List![A, B, C]>, List![A, B]> = ();
        let _: SameOp<ReverseOp<List![A, B, C]>, List![C, B, A]> = ();
        let _: SameOp<IsEmptyOp<List![]>, B1> = ();
        let _: SameOp<IsEmptyOp<List![A]>, B0> = ();
        let _: SameOp<IsEmptyOp<List![A, B, C]>, B0> = ();
        let _: SameOp<IndexOp<List![A, B, C], U0>, A> = ();
        let _: SameOp<IndexOp<List![A, B, C], U1>, B> = ();
        let _: SameOp<IndexOp<List![A, B, C], U2>, C> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeFull>, List![A, B, C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], Range<(U0, U0)>>, List![]> = ();
        let _: SameOp<IndexOp<List![A, B, C], Range<(U1, U1)>>, List![]> = ();
        let _: SameOp<IndexOp<List![A, B, C], Range<(U2, U2)>>, List![]> = ();
        let _: SameOp<IndexOp<List![A, B, C], Range<(U0, U1)>>, List![A]> = ();
        let _: SameOp<IndexOp<List![A, B, C], Range<(U2, U3)>>, List![C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], Range<(U1, U3)>>, List![B, C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeInclusive<(U0, U0)>>, List![A]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeInclusive<(U1, U1)>>, List![B]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeInclusive<(U2, U2)>>, List![C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeInclusive<(U0, U1)>>, List![A, B]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeInclusive<(U0, U2)>>, List![A, B, C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeInclusive<(U1, U2)>>, List![B, C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeTo<U0>>, List![]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeTo<U1>>, List![A]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeTo<U2>>, List![A, B]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeTo<U3>>, List![A, B, C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeToInclusive<U0>>, List![A]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeToInclusive<U1>>, List![A, B]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeToInclusive<U2>>, List![A, B, C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeFrom<U0>>, List![A, B, C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeFrom<U1>>, List![B, C]> = ();
        let _: SameOp<IndexOp<List![A, B, C], RangeFrom<U2>>, List![C]> = ();
        let _: SameOp<ReduceSumOp<List![U3, U2, U7]>, U12> = ();
        let _: SameOp<ReduceProductOp<List![U3, U2, U7]>, U42> = ();
        let _: SameOp<ZipExOp<List![]>, List![]> = ();
        let _: SameOp<ZipExOp<List![List![], List![], List![], List![]]>, List![]> = ();
        let _: SameOp<ZipExOp<List![List![A], List![B], List![C]]>, List![List![A, B, C]]> = ();
        let _: SameOp<ZipExOp<List![List![A, B], List![C, D]]>, List![List![A, C], List![B, D]]> =
            ();
    }

    #[test]
    fn map_test() {
        typ! {
            fn PlusOne<value>(value: Unsigned) -> Unsigned {
                value + 1u
            }
        }

        struct PlusOneFunc;
        impl<Value> Func<Value> for PlusOneFunc
        where
            (): PlusOne<Value>,
            Value: Unsigned,
        {
            type Output = PlusOneOp<Value>;
        }

        let _: SameOp<MapOp<List![U1, U2, U3], PlusOneFunc>, List![U2, U3, U4]> = ();
    }

    #[test]
    fn fold_test() {
        typ! {
            fn SumUp<lhs, rhs>(lhs: Unsigned, rhs: Unsigned) -> Unsigned {
                lhs + rhs
            }
        }

        struct SumUpFunc;
        impl<Lhs, Rhs> Func<(Lhs, Rhs)> for SumUpFunc
        where
            (): SumUp<Lhs, Rhs>,
            Lhs: Unsigned,
            Rhs: Unsigned,
        {
            type Output = SumUpOp<Lhs, Rhs>;
        }

        let _: SameOp<FoldOp<List![U1, U2, U3], U4, SumUpFunc>, U10> = ();
    }

    #[test]
    fn scan_test() {
        typ! {
            fn Diff<prev, curr>(prev: Unsigned, curr: Unsigned) -> Tuple2 {
                (curr, curr - prev)
            }
        }

        struct DiffFunc;
        impl<Prev, Curr> Func<(Prev, Curr)> for DiffFunc
        where
            (): Diff<Prev, Curr>,
            Prev: Unsigned,
            Curr: Unsigned,
        {
            type Output = DiffOp<Prev, Curr>;
        }

        let _: SameOp<ScanOp<List![U1, U3, U8], U0, DiffFunc>, List![U1, U2, U5]> = ();
    }

    #[test]
    fn filter_test() {
        typ! {
            fn IsNonZero<value>(value: Integer) -> Bit {
                value != 0
            }
        }

        struct IsNonZeroFunc;
        impl<Value> Func<Value> for IsNonZeroFunc
        where
            (): IsNonZero<Value>,
            Value: Integer,
        {
            type Output = IsNonZeroOp<Value>;
        }

        let _: SameOp<FilterOp<List![P1, Z0, N2, Z0], IsNonZeroFunc>, List![P1, N2]> = ();
    }

    #[test]
    fn filter_map_test() {
        typ! {
            fn FlipNegative<value>(value: Integer) -> Maybe {
                if value < 0 {
                    let neg = -value;
                    Just::<neg>
                } else {
                    Nothing
                }
            }
        }

        struct FlipNegativeFunc;
        impl<Value> Func<Value> for FlipNegativeFunc
        where
            (): FlipNegative<Value>,
            Value: Integer,
        {
            type Output = FlipNegativeOp<Value>;
        }

        let _: SameOp<FilterMapOp<List![P1, Z0, N2, Z0], FlipNegativeFunc>, List![P2]> = ();
    }
}
