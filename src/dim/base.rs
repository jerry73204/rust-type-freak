use crate::{
    common::*,
    list::{Cons, List, Nil},
};

/// Marks the list of dimensions.
pub trait Dimensions {}

impl Dimensions for DynDimensions {}
impl<Head, Tail> Dimensions for Cons<Head, Tail>
where
    Head: Dim,
    Tail: List + Dimensions,
{
}
impl Dimensions for Nil {}

/// Marks a single dimension.
pub trait Dim {}

impl Dim for Dyn {}
impl Dim for UTerm {}
impl<U, B> Dim for UInt<U, B> {}

/// The dimensions with runtime length.
pub struct DynDimensions(Vec<usize>);

/// Single dynamic dimension.
pub struct Dyn(pub usize);

typ! {
    fn PushFront<dims, dim>(dims: Dimensions, dim: Dim) -> Dimensions {
        match dims {
            DynDimensions => DynDimensions,
            #[generics(head: Dim, tail: Dimensions + List)]
            Cons::<head, tail> => {
                Cons::<dim, Cons<head, tail>>
            }
            Nil => {
                Cons::<dim, Nil>
            }
        }
    }

    fn PushBack<dims, dim>(dims: Dimensions, dim: Dim) -> Dimensions {
        match dims {
            DynDimensions => DynDimensions,
            #[generics(head: Dim, tail: Dimensions + List)]
            Cons::<head, tail> => {
                let new_tail: List = PushBack(tail, dim);
                Cons::<head, new_tail>
            }
            Nil => {
                Cons::<dim, Nil>
            }
        }
    }
}
