use super::{Cons, List, Nil};
use crate::{
    common::*,
    counter::{Counter, Curr, Next},
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

    pub fn RemoveItem<list, item, step>(list: List, item: _, step: Counter) -> List {
        match (list, step) {
            #[generics(tail: List)]
            #[capture(item)]
            (Cons::<item, tail>, Curr) => {
                tail
            }
            #[generics(head, tail: List, remaining: Counter)]
            (Cons::<head, tail>, Next::<remaining>) => {
                let new_tail = RemoveItem(tail, item, remaining);
                Cons::<head, new_tail>
            }
        }
    }

    pub fn ReplaceItem<list, prev, new, step>(list: List, prev: _, new: _, step: Counter) -> List {
        match (list, step) {
            #[generics(tail: List)]
            #[capture(prev)]
            (Cons::<prev, tail>, Curr) => {
                Cons::<new, tail>
            }
            #[generics(head, tail: List, remaining: Counter)]
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
        match list {
            #[generics(head, tail: List)]
            Cons::<head, tail> => {
                let prefix = Reverse(tail);
                PushBack(tail, head)
            }
            Nil => Nil,
        }
    }

    pub fn IndexOf<list, item, step>(list: List, item: _, step: Counter) -> Unsigned {
        match (list, step) {
            #[generics(tail: List)]
            #[capture(item)]
            (Cons::<item, tail>, Curr) => {
                0u
            }
            #[generics(head, tail: List, remaining: Counter)]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::op_aliases::*, List};
    use typenum::{U0, U1, U2, U3};

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
    // type Assert27 = AssertSame<RangeTo<List![A, B, C], U0>, List![], ()>;
    // type Assert28 = AssertSame<RangeTo<List![A, B, C], U1>, List![A], ()>;
    // type Assert29 = AssertSame<RangeTo<List![A, B, C], U2>, List![A, B], ()>;
    // type Assert30 = AssertSame<RangeTo<List![A, B, C], U3>, List![A, B, C], ()>;
    // type Assert31 = AssertSame<Range<List![A, B, C], U0, U0>, List![], ()>;
    // type Assert32 = AssertSame<Range<List![A, B, C], U0, U1>, List![A], ()>;
    // type Assert33 = AssertSame<Range<List![A, B, C], U0, U2>, List![A, B], ()>;
    // type Assert34 = AssertSame<Range<List![A, B, C], U0, U3>, List![A, B, C], ()>;
    // type Assert35 = AssertSame<Range<List![A, B, C], U1, U1>, List![], ()>;
    // type Assert36 = AssertSame<Range<List![A, B, C], U1, U2>, List![B], ()>;
    // type Assert37 = AssertSame<Range<List![A, B, C], U1, U3>, List![B, C], ()>;
    // type Assert38 = AssertSame<Range<List![A, B, C], U2, U2>, List![], ()>;
    // type Assert39 = AssertSame<Range<List![A, B, C], U2, U3>, List![C], ()>;
    // type Assert40 = AssertSame<Range<List![A, B, C], U3, U3>, List![], ()>;
    // type Assert41 = AssertSame<RangeFrom<List![A, B, C], U0>, List![A, B, C], ()>;
    // type Assert42 = AssertSame<RangeFrom<List![A, B, C], U1>, List![B, C], ()>;
    // type Assert43 = AssertSame<RangeFrom<List![A, B, C], U2>, List![C], ()>;
    // type Assert44 = AssertSame<RangeFrom<List![A, B, C], U3>, List![], ()>;
    type Assert45 = AssertSame<ZipOp<List![A, B], List![C, D]>, List![(A, C), (B, D)], ()>;
    // type Assert46<Count> = AssertSame<Insert<List![B, C], B, Count, A>, List![A, B, C], ()>;
    // type Assert47<Count> = AssertSame<Insert<List![B, C], C, Count, A>, List![B, A, C], ()>;
    type Assert48 = AssertSame<PopFrontOp<List![A, B, C]>, List![B, C], ()>;
    type Assert49 = AssertSame<PopBackOp<List![A, B, C]>, List![A, B], ()>;
    // type Assert50<Counters> = AssertSame<RemoveMany<List![A], List![], Counters>, List![A], ()>;
    // type Assert51<Counters> = AssertSame<RemoveMany<List![A], List![A], Counters>, List![], ()>;
    // type Assert52<Counters> =
    //     AssertSame<RemoveMany<List![A, B, C], List![A, C], Counters>, List![B], ()>;
    // type Assert53<Counters> = AssertSame<Swap<List![A, B, C], A, B, Counters>, List![B, A, C], ()>;
    // type Assert54<Counters> = AssertSame<Swap<List![A, B, C], A, C, Counters>, List![C, B, A], ()>;

    // TODO: test ForEach and Fold

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
        // let _: Assert13<_> = ();
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
        // let _: Assert26<_> = ();
        // let _: Assert27 = ();
        // let _: Assert28 = ();
        // let _: Assert29 = ();
        // let _: Assert30 = ();
        // let _: Assert31 = ();
        // let _: Assert32 = ();
        // let _: Assert33 = ();
        // let _: Assert34 = ();
        // let _: Assert35 = ();
        // let _: Assert36 = ();
        // let _: Assert37 = ();
        // let _: Assert38 = ();
        // let _: Assert39 = ();
        // let _: Assert40 = ();
        // let _: Assert41 = ();
        // let _: Assert42 = ();
        // let _: Assert43 = ();
        // let _: Assert44 = ();
        let _: Assert45 = ();
        // let _: Assert46<_> = ();
        // let _: Assert47<_> = ();
        let _: Assert48 = ();
        let _: Assert49 = ();
        // let _: Assert50<_> = ();
        // let _: Assert51<_> = ();
        // let _: Assert52<_> = ();
        // let _: Assert53<_> = ();
        // let _: Assert54<_> = ();
        let _: AssertSame<IsEmptyOp<List![]>, B1, ()> = ();
        let _: AssertSame<IsEmptyOp<List![A]>, B0, ()> = ();
        let _: AssertSame<IsEmptyOp<List![A, B, C]>, B0, ()> = ();
    }
}
