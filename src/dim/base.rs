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
    Tail: DimsList,
{
}
impl Dimensions for Nil {}

pub trait DimsList
where
    Self: List,
{
}
impl<Head, Tail> DimsList for Cons<Head, Tail>
where
    Head: Dim,
    Tail: DimsList,
{
}
impl DimsList for Nil {}

/// Marks a single dimension.
pub trait Dim {}

impl Dim for Dyn {}
impl Dim for UTerm {}
impl<U, B> Dim for UInt<U, B> {}

/// The dimensions with runtime length.
pub struct DynDimensions(Vec<usize>);

/// Single dynamic dimension.
pub struct Dyn(pub usize);

pub type Dims2D<P, Q> = Cons<P, Cons<Q, Nil>>;
