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

    // add

    impl Add<Dyn> for Dyn {
        type Output = Dyn;

        fn add(self, rhs: Dyn) -> Self::Output {
            Dyn(self.0 + rhs.0)
        }
    }

    impl<U, B> Add<Dyn> for UInt<U, B>
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn add(self, rhs: Dyn) -> Self::Output {
            Dyn(Self::USIZE + rhs.0)
        }
    }

    impl<U, B> Add<UInt<U, B>> for Dyn
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn add(self, _rhs: UInt<U, B>) -> Self::Output {
            Dyn(self.0 + UInt::<U, B>::USIZE)
        }
    }

    impl Add<Dyn> for UTerm {
        type Output = Dyn;

        fn add(self, rhs: Dyn) -> Self::Output {
            rhs
        }
    }

    impl Add<UTerm> for Dyn {
        type Output = Dyn;

        fn add(self, _rhs: UTerm) -> Self::Output {
            self
        }
    }

    // sub

    impl Sub<Dyn> for Dyn {
        type Output = Dyn;

        fn sub(self, rhs: Dyn) -> Self::Output {
            Dyn(self.0 - rhs.0)
        }
    }

    impl<U, B> Sub<Dyn> for UInt<U, B>
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn sub(self, rhs: Dyn) -> Self::Output {
            Dyn(Self::USIZE - rhs.0)
        }
    }

    impl<U, B> Sub<UInt<U, B>> for Dyn
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn sub(self, _rhs: UInt<U, B>) -> Self::Output {
            Dyn(self.0 - UInt::<U, B>::USIZE)
        }
    }

    impl Sub<Dyn> for UTerm {
        type Output = Dyn;

        fn sub(self, rhs: Dyn) -> Self::Output {
            Dyn(0 - rhs.0)
        }
    }

    impl Sub<UTerm> for Dyn {
        type Output = Dyn;

        fn sub(self, _rhs: UTerm) -> Self::Output {
            self
        }
    }

    // mul

    impl Mul<Dyn> for Dyn {
        type Output = Dyn;

        fn mul(self, rhs: Dyn) -> Self::Output {
            Dyn(self.0 * rhs.0)
        }
    }

    impl<U, B> Mul<Dyn> for UInt<U, B>
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn mul(self, rhs: Dyn) -> Self::Output {
            Dyn(Self::USIZE * rhs.0)
        }
    }

    impl<U, B> Mul<UInt<U, B>> for Dyn
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn mul(self, _rhs: UInt<U, B>) -> Self::Output {
            Dyn(self.0 * UInt::<U, B>::USIZE)
        }
    }

    impl Mul<Dyn> for UTerm {
        type Output = UTerm;

        fn mul(self, _rhs: Dyn) -> Self::Output {
            UTerm
        }
    }

    impl Mul<UTerm> for Dyn {
        type Output = UTerm;

        fn mul(self, rhs: UTerm) -> Self::Output {
            rhs
        }
    }

    // div

    impl Div<Dyn> for Dyn {
        type Output = Dyn;

        fn div(self, rhs: Dyn) -> Self::Output {
            Dyn(self.0 / rhs.0)
        }
    }

    impl<U, B> Div<Dyn> for UInt<U, B>
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn div(self, rhs: Dyn) -> Self::Output {
            Dyn(Self::USIZE - rhs.0)
        }
    }

    impl<U, B> Div<UInt<U, B>> for Dyn
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = Dyn;

        fn div(self, _rhs: UInt<U, B>) -> Self::Output {
            Dyn(self.0 / UInt::<U, B>::USIZE)
        }
    }

    impl Div<Dyn> for UTerm {
        type Output = Dyn;

        fn div(self, rhs: Dyn) -> Self::Output {
            Dyn(0 / rhs.0)
        }
    }
}

// aliases

pub type Dims2D<P, Q> = Cons<P, Cons<Q, Nil>>;
pub type Dims3D<P, Q, R> = Cons<P, Cons<Q, Cons<R, Nil>>>;
