use super::{Dim, Dimensions, Dims2D, DimsList, Dyn, DynDimensions};
use crate::{
    common::*,
    list::{Cons, Extend, Index, Len, List, Nil, ReduceProduct, Reverse},
    numeric::UnsignedIntegerDiv,
};
use typenum::U1;

typ! {
    pub fn PushFront<dims, dim>(dims: Dimensions, dim: Dim) -> Dimensions {
        match dims {
            DynDimensions => DynDimensions,
            #[generics(head: Dim, tail: DimsList)]
            Cons::<head, tail> => {
                Cons::<dim, Cons<head, tail>>
            }
            Nil => {
                Cons::<dim, Nil>
            }
        }
    }

    pub fn PushBack<dims, dim>(dims: Dimensions, dim: Dim) -> Dimensions {
        match dims {
            DynDimensions => DynDimensions,
            #[generics(head: Dim, tail: DimsList)]
            Cons::<head, tail> => {
                let new_tail: List = PushBack(tail, dim);
                Cons::<head, new_tail>
            }
            Nil => {
                Cons::<dim, Nil>
            }
        }
    }

    pub fn MatrixTranspose<dims>(dims: Dimensions) -> Dimensions {
        match dims {
            DynDimensions => DynDimensions,
            #[generics(p: Dim, q: Dim)]
            (Dims2D::<p, q>, DynDimensions) => Dims2D::<q, p>,
        }
    }


    pub fn MatrixDot<lhs, rhs>(lhs: Dimensions, rhs: Dimensions) -> Dimensions {
        match (lhs, rhs) {
            #[capture(rhs)]
            (DynDimensions, rhs) => DynDimensions,
            #[generics(p: Dim, q: Dim)]
            (Dims2D::<p, q>, DynDimensions) => DynDimensions,
            #[generics(p: Dim, r: Dim, uint: Unsigned, bit: Bit)]
            (Dims2D::<p, UInt<uint, bit>>, Dims2D::<UInt<uint, bit>, r>) => Dims2D::<p, r>,
            #[generics(p: Dim, r: Dim)]
            (Dims2D::<p, UTerm>, Dims2D::<UTerm, r>) => Dims2D::<p, r>,
            #[generics(p: Dim, r: Dim, uint: Unsigned, bit: Bit)]
            (Dims2D::<p, Dyn>, Dims2D::<UInt<uint, bit>, r>) => Dims2D::<p, r>,
            #[generics(p: Dim, r: Dim)]
            (Dims2D::<p, Dyn>, Dims2D::<UTerm, r>) => Dims2D::<p, r>,
            #[generics(p: Dim, r: Dim, uint: Unsigned, bit: Bit)]
            (Dims2D::<p, UInt<uint, bit>>, Dims2D::<Dyn, r>) => Dims2D::<p, r>,
            #[generics(p: Dim, r: Dim)]
            (Dims2D::<p, UTerm>, Dims2D::<Dyn, r>) => Dims2D::<p, r>,
            #[generics(p: Dim, r: Dim)]
            (Dims2D::<p, Dyn>, Dims2D::<Dyn, r>) => Dims2D::<p, r>,
        }
    }

    // buggy
    // pub fn Flatten<input, start, end>(input: Dimensions, start: Dim, end: Dim) -> Dimensions {
    //     let is_dyn = match (input, start, end) {
    //         #[capture(start, end)]
    //         (DynDimensions, start, end) => true,
    //         #[generics(head: Dim, tail: DimsList)]
    //         #[capture(end)]
    //         (Cons::<head, tail>, Dyn, end) => true,
    //         #[generics(head: Dim, tail: DimsList)]
    //         (Cons::<head, tail>, UTerm, Dyn) => true,
    //         #[generics(head: Dim, tail: DimsList)]
    //         (Cons::<head, tail>, UTerm, UTerm) => false,
    //         #[generics(head: Dim, tail: DimsList, uint: Unsigned, bit: Bit)]
    //         (Cons::<head, tail>, UTerm, UInt::<uint, bit>) => false,
    //         #[generics(head: Dim, tail: DimsList, uint: Unsigned, bit: Bit)]
    //         (Cons::<head, tail>, UInt::<uint, bit>, Dyn) => true,
    //         #[generics(head: Dim, tail: DimsList, uint1: Unsigned, bit1: Bit, uint2: Unsigned, bit2: Bit)]
    //         (Cons::<head, tail>, UInt::<uint1, bit1>, UInt::<uint2, bit2>) => false,
    //     };

    //     if is_dyn {
    //         DynDimensions
    //     } else {
    //         let heading: DimsList = Index(input, RangeTo::<start>);
    //         let trailing: DimsList = if end == Len(input) {
    //             Nil
    //         } else {
    //             Index(input, RangeFrom::<end>)
    //         };
    //         let product: Dim = ReduceProduct(Index(input, Range::<(start, end)>));
    //         Extend(heading, Cons::<product, trailing>)
    //     }
    // }

    pub fn PyTorchBroadcast<lhs, rhs>(lhs: Dimensions, rhs: Dimensions) -> Dimensions {
        match (lhs, rhs) {
            #[capture(rhs)]
            (DynDimensions, rhs) => DynDimensions,
            #[generics(dim: Dim, tail: DimsList)]
            (Cons::<dim, tail>, DynDimensions) => DynDimensions,
            #[generics(ldim: Dim, ltail: DimsList, rdim: Dim, rtail: DimsList)]
            (Cons::<ldim, ltail>, Cons::<rdim, rtail>) => {
                Reverse(PyTorchBroadcastRecursive(Reverse(lhs), Reverse(rhs)))
            }
        }
    }

    fn PyTorchBroadcastRecursive<lhs, rhs>(lhs: DimsList, rhs: DimsList) -> DimsList
    {
        match (lhs, rhs) {
            (Nil, Nil) => Nil,
            #[generics(dim: Dim, tail: DimsList)]
            (Nil, Cons::<dim, tail>) => rhs,
            #[generics(dim: Dim, tail: DimsList)]
            (Cons::<dim, tail>, Nil) => lhs,
            #[generics(ldim: Dim, ltail: DimsList, rdim: Dim, rtail: DimsList)]
            (Cons::<ldim, ltail>, Cons::<rdim, rtail>) => {
                let dim: Dim = match (ldim, rdim) {
                    #[capture(rdim)]
                    (Dyn, rdim) => Dyn,
                    #[generics(uint: Unsigned, bit: Bit)]
                    (UInt::<uint, bit>, Dyn) => Dyn,
                    #[generics(uint: Unsigned, bit: Bit)]
                    (U1, UInt::<uint, bit>) => rdim,
                    #[generics(uint: Unsigned, bit1: Unsigned, bit2: Bit)]
                    (UInt::<UInt<uint, bit1>, bit2>, U1) => ldim,
                    #[generics(uint: Unsigned, bit1: Unsigned, bit2: Bit)]
                    (UInt::<UInt<uint, bit1>, bit2>, UInt::<UInt<uint, bit1>, bit2>) => ldim,
                };
                let tail: DimsList = PyTorchBroadcastRecursive(ltail, rtail);
                Cons::<dim, tail>
            }
        }
    }

    pub fn ConvDim<size, padding, dilation, ksize, stride>(size: Unsigned, padding: Unsigned, dilation: Unsigned, ksize: Unsigned, stride: Unsigned) -> Unsigned {
        UnsignedIntegerDiv(size + 2u * padding - dilation * (ksize - 1u) - 1u, stride) + 1u
    }
}

typ! {
    pub fn Cat<inputs, index>(inputs: List, index: Dim) -> Dimensions {
        match (CheckDynDimensions(inputs), index) {
            #[capture(index)]
            (B1, index) => DynDimensions,
            (B0, Dyn) => DynDimensions,
            (B0, UTerm) => MergeLeadingDims(Nil, inputs, index),
            #[generics(uint: Unsigned, bit: Bit)]
            (B0, UInt::<uint, bit>) => MergeLeadingDims(Nil, inputs, index),
        }
    }

    fn MergeLeadingDims<saved, remaining, index>(saved: DimsList, remaining: List, index: Unsigned) -> DimsList {
        if index == 0u {
            let new_dim = SumDims(remaining);
            let new_remaining = RemoveDims(Nil, remaining);
            let new_saved = Cons::<new_dim, saved>;
            MergeTrailingDims(new_saved, new_remaining)
        } else {
            let expect = ExtractFirstDim(Dyn, remaining);
            let new_remaining = RemoveExpectedDims(Nil, remaining, expect);
            let new_saved = Cons::<expect, saved>;
            MergeLeadingDims(new_saved, new_remaining, index - 1u)
        }
    }

    fn MergeTrailingDims<saved, remaining>(saved: DimsList, remaining: List) -> DimsList {
        match remaining {
            #[generics(dim: Dim, dims_tail: DimsList, tail: List)]
            Cons::<Cons::<dim, dims_tail>, tail> => {
                let expect = ExtractFirstDim(Dyn, remaining);
                let new_remaining = RemoveExpectedDims(Nil, remaining, expect);
                let new_saved = Cons::<expect, saved>;
                MergeTrailingDims(new_saved, new_remaining)
            }
            #[generics(tail: List)]
            Cons::<Nil, tail> => {
                AssertAllEmpty(remaining);
                Reverse(saved)
            }
        }
    }

    fn AssertAllEmpty<remaining>(remaining: List) {
        match remaining {
            #[generics(tail: List)]
            Cons::<Nil, tail> => AssertAllEmpty(tail),
            Nil => (),
        }
    }

    fn CheckDynDimensions<remaining>(remaining: List) -> Bit {
        match remaining {
            #[generics(tail: List)]
            Cons::<DynDimensions, tail> => true,
            #[generics(dim: Dim, dims_tail: DimsList, tail: List)]
            Cons::<Cons::<dim, dims_tail>, tail> => CheckDynDimensions(tail),
            Nil => false,
        }
    }

    fn ExtractFirstDim<expect, remaining>(expect: Dim, remaining: List) -> Dim {
        match remaining {
            #[generics(dim: Dim, dims_tail: DimsList, tail: List)]
            Cons::<Cons<dim, dims_tail>, tail> => {
                match (expect, dim) {
                    #[capture(dim)]
                    (Dyn, dim) => ExtractFirstDim(dim, tail),
                    (UTerm, Dyn) => ExtractFirstDim(expect, tail),
                    #[generics(uint: Unsigned, bit: Bit)]
                    (UInt::<uint, bit>, Dyn) => ExtractFirstDim(expect, tail),
                    (UTerm, UTerm) => ExtractFirstDim(expect, tail),
                    #[generics(uint: Unsigned, bit: Bit)]
                    (UInt::<uint, bit>, UInt::<uint, bit>) => ExtractFirstDim(expect, tail),
                }
            }
            Nil => expect,
        }
    }

    fn RemoveDims<saved, remaining>(saved: List, remaining: List) -> List {
        match remaining {
            #[generics(dim: Dim, dims_tail: DimsList, tail: List)]
            Cons::<Cons::<dim, dims_tail>, tail> => RemoveDims(Cons::<dims_tail, saved>, tail),
            Nil => Reverse(saved),
        }
    }

    fn RemoveExpectedDims<saved, remaining, expect>(saved: List, remaining: List, expect: Dim) -> List {
        match remaining {
            #[generics(dim: Dim, dims_tail: DimsList, tail: List)]
            Cons::<Cons::<dim, dims_tail>, tail> => {
                match (expect, dim) {
                    #[capture(dim)]
                    (Dyn, Dyn) => RemoveExpectedDims(Cons::<dims_tail, saved>, tail, expect),
                    (UTerm, Dyn) => RemoveExpectedDims(Cons::<dims_tail, saved>, tail, expect),
                    #[generics(uint: Unsigned, bit: Bit)]
                    (UInt::<uint, bit>, Dyn) => RemoveExpectedDims(Cons::<dims_tail, saved>, tail, expect),
                    (UTerm, UTerm) => RemoveExpectedDims(Cons::<dims_tail, saved>, tail, expect),
                    #[generics(uint: Unsigned, bit: Bit)]
                    (UInt::<uint, bit>, UInt::<uint, bit>) => RemoveExpectedDims(Cons::<dims_tail, saved>, tail, expect),
                }
            }
            Nil => Reverse(saved),
        }
    }

    fn SumDims<remaining>(remaining: List) -> Dim {
        match remaining {
            #[generics(dim: Dim, dims_tail: DimsList, tail: List)]
            Cons::<Cons::<dim, dims_tail>, tail> => {
                let remaining_sum = SumDims(tail);
                match (remaining_sum, dim) {
                    #[capture(dim)]
                    (Dyn, dim) => Dyn,
                    (UTerm, Dyn) => Dyn,
                    (UTerm, UTerm) => UTerm,
                    #[generics(uint: Unsigned, bit: Bit)]
                    (UTerm, UInt::<uint, bit>) => dim,
                    #[generics(uint: Unsigned, bit: Bit)]
                    (UInt::<uint, bit>, Dyn) => Dyn,
                    #[generics(uint: Unsigned, bit: Bit)]
                    (UInt::<uint, bit>, UTerm) => remaining_sum,
                    #[generics(uint1: Unsigned, bit1: Bit, uint2: Unsigned, bit2: Bit)]
                    (UInt::<uint1, bit1>, UInt::<uint2, bit2>) => dim + remaining_sum,
                }
            }
            Nil => UTerm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::op_aliases::AssertSame, Dims, List};
    use typenum::consts::*;

    fn test() {
        // let _: AssertSame<FlattenOp<Dims![1, 2, 3], U0, U3>, U6, ()> = ();
        let _: AssertSame<
            CatOp<List![Dims![1, 2, 3], DynDimensions], tyuint!(1)>,
            DynDimensions,
            (),
        > = ();
        let _: AssertSame<CatOp<List![Dims![1, 2, 3], Dims![1, 5, 3]], Dyn>, Dims![?], ()> = ();
        let _: AssertSame<CatOp<List![Dims![2], Dims![3]], tyuint!(0)>, Dims![5], ()> = ();
        let _: AssertSame<CatOp<List![Dims![2], Dims![_]], tyuint!(0)>, Dims![_], ()> = ();
        let _: AssertSame<CatOp<List![Dims![_], Dims![_]], tyuint!(0)>, Dims![_], ()> = ();
        let _: AssertSame<
            CatOp<List![Dims![2, 5, 3], Dims![2, 1, 3]], tyuint!(1)>,
            Dims![2, 6, 3],
            (),
        > = ();
        let _: AssertSame<
            CatOp<List![Dims![_, 5, _, 3], Dims![2, 1, _, _]], tyuint!(1)>,
            Dims![2, 6, _, 3],
            (),
        > = ();
        let _: AssertSame<
            CatOp<List![Dims![_, 7, _, 3], Dims![2, 7, 8, _]], tyuint!(2)>,
            Dims![2, 7, _, 3],
            (),
        > = ();
        let _: AssertSame<
            CatOp<List![Dims![_, 7, 1, 3], Dims![2, _, 8, _], Dims![2, 7, 4, _]], tyuint!(2)>,
            Dims![2, 7, 13, 3],
            (),
        > = ();
    }
}
