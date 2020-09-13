use super::{Dim, Dimensions, Dims2D, DimsList, Dyn, DynDimensions};
use crate::{
    common::*,
    list::{Cons, List, Nil, Reverse},
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

    pub fn MatrixTranspose<dims>(dims: Dimensions) -> Dimensions {
        match dims {
            DynDimensions => DynDimensions,
            #[generics(p: Dim, q: Dim)]
            (Dims2D::<p, q>, DynDimensions) => Dims2D::<q, p>,
        }
    }

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

    pub fn PyTorchBroadcastRecursive<lhs, rhs>(lhs: Dimensions, rhs: Dimensions) -> Dimensions
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
}
