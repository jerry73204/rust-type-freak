use crate::{
    common::*,
    list::{Cons, List, Nil},
};

pub use dyn_dim::*;

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

mod dyn_dim {
    use super::*;

    /// Single dynamic dimension.
    pub struct Dyn(pub usize);

    impl Add<Dyn> for Dyn {
        type Output = Dyn;

        fn add(self, rhs: Dyn) -> Self::Output {
            Dyn(self.0 + rhs.0)
        }
    }

    impl<U, B> Add<UInt<U, B>> for Dyn
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn add(self, rhs: UInt<U, B>) -> Self::Output {
            todo!()
        }
    }

    impl Add<UTerm> for Dyn {
        type Output = Dyn;

        fn add(self, rhs: UTerm) -> Self::Output {
            self
        }
    }

    impl Sub<Dyn> for Dyn {
        type Output = Dyn;

        fn sub(self, rhs: Dyn) -> Self::Output {
            Dyn(self.0 + rhs.0)
        }
    }

    impl<U, B> Sub<UInt<U, B>> for Dyn
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn sub(self, rhs: UInt<U, B>) -> Self::Output {
            todo!()
        }
    }

    impl Sub<UTerm> for Dyn {
        type Output = Dyn;

        fn sub(self, rhs: UTerm) -> Self::Output {
            self
        }
    }

    impl Mul<Dyn> for Dyn {
        type Output = Dyn;

        fn mul(self, rhs: Dyn) -> Self::Output {
            Dyn(self.0 + rhs.0)
        }
    }

    impl<U, B> Mul<UInt<U, B>> for Dyn
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn mul(self, rhs: UInt<U, B>) -> Self::Output {
            todo!()
        }
    }

    impl Mul<UTerm> for Dyn {
        type Output = Dyn;

        fn mul(self, rhs: UTerm) -> Self::Output {
            self
        }
    }
    impl Div<Dyn> for Dyn {
        type Output = Dyn;

        fn div(self, rhs: Dyn) -> Self::Output {
            Dyn(self.0 + rhs.0)
        }
    }

    impl<U, B> Div<UInt<U, B>> for Dyn
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn div(self, rhs: UInt<U, B>) -> Self::Output {
            todo!()
        }
    }

    impl Div<UTerm> for Dyn {
        type Output = Dyn;

        fn div(self, rhs: UTerm) -> Self::Output {
            self
        }
    }
}

// aliases

pub type Dims2D<P, Q> = Cons<P, Cons<Q, Nil>>;
pub type Dims3D<P, Q, R> = Cons<P, Cons<Q, Cons<R, Nil>>>;
