use crate::{
    common::*,
    list::{Cons, List, Nil},
};

pub use base::*;
pub use ops::*;

mod base {
    use super::*;

    pub trait BitSet
    where
        Self: List,
    {
    }

    impl<B, Tail> BitSet for Cons<B, Tail>
    where
        B: Bit,
        Tail: BitSet,
    {
    }

    impl BitSet for Nil {}

}

mod ops {
    use super::*;

    typ! {
        pub fn Length<input>(input: BitSet) -> Unsigned {
            match input {
                #[generics(bit: Bit, tail: BitSet)]
                Cons::<bit, tail> => Length(tail) + 1u,
                Nil => 0u,
            }
        }

        pub fn BitSetAdd<lhs, rhs>(lhs: BitSet, rhs: BitSet) -> BitSet {
            BitSetAddRecursive(lhs, rhs, B0)
        }

        fn BitSetAddRecursive<lhs, rhs, carry>(lhs: BitSet, rhs: BitSet, carry: Bit) -> BitSet {
            match (lhs, rhs) {
                (Nil, Nil) => Nil,
                #[generics(lbit: Bit, ltail: BitSet, rbit: Bit, rtail: BitSet)]
                (Cons::<lbit, ltail>, Cons::<rbit, rtail>) => {
                    let output: Bit = lbit.BitXor(rbit).BitXor(carry);
                    let new_carry: Bit = lbit & rbit | lbit & carry | rbit & carry;
                    let new_tail = BitSetAddRecursive(ltail, rtail, new_carry);
                    Cons::<output, new_tail>
                }
            }
        }

        pub fn Truncate<input, len>(input: BitSet, len: Unsigned) -> BitSet {
            TruncateRecursive(input, input, len)
        }

        fn TruncateRecursive<saved, remaining, len>(saved: BitSet, remaining: BitSet, len: Unsigned) -> BitSet {
            if len == 0u {
                Reverse(saved)
            } else {
                match remaining {
                    #[generics(bit: Bit, tail: BitSet)]
                    Cons::<bit, tail> => {
                        let new_saved = Cons::<bit, saved>;
                        let new_remaining = tail;
                        let new_len: Unsigned = len - 1u;
                        TruncateRecursive(new_saved, new_remaining, new_len)
                    }
                }
            }
        }

        pub fn Reverse<input>(input: BitSet) -> BitSet {
            ReverseRecursive(Nil, input)
        }

        fn ReverseRecursive<saved, remaining>(saved: BitSet, remaining: BitSet) -> BitSet {
            match remaining {
                #[generics(bit: Bit, tail: BitSet)]
                Cons::<bit, tail> => {
                    let new_saved = Cons::<bit, saved>;
                    let new_remaining = tail;
                    ReverseRecursive(new_saved, new_remaining)
                }
                Nil => saved,
            }
        }
    }
}
