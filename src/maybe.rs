//! Trait-level equivalences to [Option](std::option::Option).
//!
//! The trait [Maybe](crate::maybe::Maybe) corresponds to [Option](std::option::Option). The types
//! [Just](crate::maybe::Just) and [Nothing](crate::maybe::Nothing) corresponds `Some` and `None` respectively.
//!
//! ```rust
//! use typenum::consts::*;
//! use type_freak::maybe::{Maybe, Just, Nothing, UnwrapOutput, UnwrapOrOutput};
//!
//! type Opt1 = Just<U3>;
//! type Opt2 = Nothing;
//!
//! type Val1 = UnwrapOutput<Opt1>;       // U3
//! type Val2 = UnwrapOrOutput<Opt1, U0>; // U3
//! type Val3 = UnwrapOrOutput<Opt2, U0>; // U0
//! ```

use crate::functional::{ApplyFunctor, Functor};
use std::marker::PhantomData;

// maybe def

/// A trait analogous to [Option](std::option::Option).
pub trait Maybe {}

// just def

/// A type analogous to `Some`.
pub struct Just<T> {
    _phantom: PhantomData<T>,
}

impl<T> Maybe for Just<T> {}

// nothing def

/// A type analogous to `None`.
pub struct Nothing;

impl Maybe for Nothing {}

// unwrap op

/// A type operator that unwraps [Just<T>](Just).
pub trait Unwrap
where
    Self: Maybe,
{
    type Output;
}

pub type UnwrapOutput<T> = <T as Unwrap>::Output;

impl<T> Unwrap for Just<T> {
    type Output = T;
}

// unwrap or default op

/// A type operator that unwraps [Just<T>](Just),
/// or derives to default type for [Nothing].
pub trait UnwrapOr<DefaultValue>
where
    Self: Maybe,
{
    type Output;
}

pub type UnwrapOrOutput<T, DefaultValue> = <T as UnwrapOr<DefaultValue>>::Output;

impl<T, DefaultValue> UnwrapOr<DefaultValue> for Just<T> {
    type Output = T;
}

impl<DefaultValue> UnwrapOr<DefaultValue> for Nothing {
    type Output = DefaultValue;
}

// map the value of maybe

/// Maps a [Maybe] type by a [Functor].
pub trait MaybeMap<Func>
where
    Self: Maybe,
    Self::Output: Maybe,
{
    type Output;
}

pub type MaybeMapOutput<MaybeInput, Func> = <MaybeInput as MaybeMap<Func>>::Output;

impl<Func> MaybeMap<Func> for Nothing {
    type Output = Nothing;
}

impl<Func, T> MaybeMap<Func> for Just<T>
where
    Func: Functor<T>,
{
    type Output = Just<ApplyFunctor<Func, T>>;
}

/// Filters a [Maybe] type by a [Functor].
pub trait MaybeFilter<Func>
where
    Self: Maybe,
    Self::Output: Maybe,
{
    type Output;
}

pub type MaybeFilterOutput<MaybeInput, Func> = <MaybeInput as MaybeFilter<Func>>::Output;

impl<Func> MaybeFilter<Func> for Nothing {
    type Output = Nothing;
}

impl<Func, T> MaybeFilter<Func> for Just<T>
where
    Func: Functor<T>,
    Func::Output: Maybe,
{
    type Output = ApplyFunctor<Func, T>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        boolean::Boolean,
        control::{IfElsePredicate, IfElsePredicateOutput, IfSameOutput},
    };
    use typenum::{consts::*, GrEq, IsGreaterOrEqual, Unsigned};

    // unwrap
    type Opt1 = Just<U3>;
    type Opt2 = Nothing;

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert1 = AssertSame<UnwrapOutput<Opt1>, U3>;
    type Assert2 = AssertSame<UnwrapOrOutput<Opt1, U0>, U3>;
    type Assert3 = AssertSame<UnwrapOrOutput<Opt2, U0>, U0>;

    // map
    struct BoxFunc;

    impl<Input> Functor<Input> for BoxFunc {
        type Output = Box<Input>;
    }

    type Assert4 = IfSameOutput<(), MaybeMapOutput<Just<i8>, BoxFunc>, Just<Box<i8>>>;
    type Assert5 = IfSameOutput<(), MaybeMapOutput<Nothing, BoxFunc>, Nothing>;

    // filter
    struct ThresholdFunc;

    impl<Input> Functor<Input> for ThresholdFunc
    where
        Input: Unsigned + IsGreaterOrEqual<U4>,
        GrEq<Input, U4>: Boolean,
        Just<Input>: IfElsePredicate<GrEq<Input, U4>, Nothing>,
    {
        type Output = IfElsePredicateOutput<Just<Input>, GrEq<Input, U4>, Nothing>;
    }

    type Assert6 = IfSameOutput<(), MaybeFilterOutput<Just<U8>, ThresholdFunc>, Just<U8>>;
    type Assert7 = IfSameOutput<(), MaybeFilterOutput<Just<U2>, ThresholdFunc>, Nothing>;
    type Assert8 = IfSameOutput<(), MaybeFilterOutput<Nothing, ThresholdFunc>, Nothing>;

    #[test]
    fn maybe_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
    }
}
