//! Trait-level equivalences to [Option](std::option::Option).
//!
//! The trait [Maybe](crate::maybe::Maybe) corresponds to [Option](std::option::Option). The types
//! [Just](crate::maybe::Just) and [Nothing](crate::maybe::Nothing) corresponds `Some` and `None` respectively.
//!
//! ```rust
//! use typenum::consts::*;
//! use type_freak::maybe::{Maybe, Just, Nothing, Unwrap, UnwrapOr};
//!
//! type Opt1 = Just<U3>;
//! type Opt2 = Nothing;
//!
//! type Val1 = Unwrap<Opt1>;       // U3
//! type Val2 = UnwrapOr<Opt1, U0>; // U3
//! type Val3 = UnwrapOr<Opt2, U0>; // U0
//! ```

use crate::functional::{ApplicativeFunctor, ApplyFunctor, FMapFunctor, Functor};
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

/// A [Functor] that unwraps [Just<T>](Just).
pub struct UnwrapFunctor {}

pub type Unwrap<T> = ApplyFunctor<UnwrapFunctor, T>;

impl<T> Functor<Just<T>> for UnwrapFunctor {
    type Output = T;
}

// unwrap or default op

/// A [Functor] that unwraps [Just<T>](Just), or returns `Defaultvalue` if [Nothing].
pub struct UnwrapOrFunctor<DefaultValue> {
    _phantom: PhantomData<DefaultValue>,
}

pub type UnwrapOr<T, DefaultValue> = ApplyFunctor<UnwrapOrFunctor<DefaultValue>, T>;

impl<T, DefaultValue> Functor<Just<T>> for UnwrapOrFunctor<DefaultValue> {
    type Output = T;
}

impl<DefaultValue> Functor<Nothing> for UnwrapOrFunctor<DefaultValue> {
    type Output = DefaultValue;
}

// map the value of maybe

/// A [Functor] that maps a [Maybe] type by a [Functor].
pub struct MaybeMapFunctor<Func> {
    _phantom: PhantomData<Func>,
}

pub type MaybeMap<Input, Func> = ApplyFunctor<MaybeMapFunctor<Func>, Input>;

impl<Func> Functor<Nothing> for MaybeMapFunctor<Func> {
    type Output = Nothing;
}

impl<Func, T> Functor<Just<T>> for MaybeMapFunctor<Func>
where
    Func: Functor<T>,
{
    type Output = Just<ApplyFunctor<Func, T>>;
}

// filter

/// A [Functor] that filters a [Maybe] type by a [Functor].
pub struct MaybeFilterFunctor<Func> {
    _phantom: PhantomData<Func>,
}

pub type MaybeFilter<Input, Func> = ApplyFunctor<MaybeFilterFunctor<Func>, Input>;

impl<Func> Functor<Nothing> for MaybeFilterFunctor<Func> {
    type Output = Nothing;
}

impl<Func, T> Functor<Just<T>> for MaybeFilterFunctor<Func>
where
    Func: Functor<T>,
    Func::Output: Maybe,
{
    type Output = ApplyFunctor<Func, T>;
}

// impl FMap for Maybe

impl<Func> Functor<Nothing> for FMapFunctor<Func> {
    type Output = Nothing;
}

impl<T, Func> Functor<Just<T>> for FMapFunctor<Func>
where
    MaybeMapFunctor<Func>: Functor<Just<T>>,
{
    type Output = MaybeMap<Just<T>, Func>;
}

// Applicative

impl Functor<Nothing> for ApplicativeFunctor<Nothing> {
    type Output = Nothing;
}

impl<Func> Functor<Just<Func>> for ApplicativeFunctor<Nothing> {
    type Output = Nothing;
}

impl<Value> Functor<Nothing> for ApplicativeFunctor<Just<Value>> {
    type Output = Nothing;
}

impl<Func, Value> Functor<Just<Func>> for ApplicativeFunctor<Just<Value>>
where
    Func: Functor<Value>,
{
    type Output = Just<ApplyFunctor<Func, Value>>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        boolean::Boolean,
        control::{IfElsePredicate, IfElsePredicateOutput, IfSameOutput},
        functional::{Applicative, FMap},
        numeric::AddOneFunctor,
    };
    use typenum::{consts::*, GrEq, IsGreaterOrEqual, Unsigned};

    // unwrap
    type Opt1 = Just<U3>;
    type Opt2 = Nothing;

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert1 = AssertSame<Unwrap<Opt1>, U3>;
    type Assert2 = AssertSame<UnwrapOr<Opt1, U0>, U3>;
    type Assert3 = AssertSame<UnwrapOr<Opt2, U0>, U0>;

    // map
    struct BoxFunc;

    impl<Input> Functor<Input> for BoxFunc {
        type Output = Box<Input>;
    }

    type Assert4 = IfSameOutput<(), MaybeMap<Just<i8>, BoxFunc>, Just<Box<i8>>>;
    type Assert5 = IfSameOutput<(), MaybeMap<Nothing, BoxFunc>, Nothing>;

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

    type Assert6 = IfSameOutput<(), MaybeFilter<Just<U8>, ThresholdFunc>, Just<U8>>;
    type Assert7 = IfSameOutput<(), MaybeFilter<Just<U2>, ThresholdFunc>, Nothing>;
    type Assert8 = IfSameOutput<(), MaybeFilter<Nothing, ThresholdFunc>, Nothing>;

    // FMap
    type Assert9 = IfSameOutput<(), FMap<Nothing, AddOneFunctor>, Nothing>;
    type Assert10 = IfSameOutput<(), FMap<Just<U8>, AddOneFunctor>, Just<U9>>;

    // Applicative
    type Assert11 = IfSameOutput<(), Applicative<Nothing, Nothing>, Nothing>;
    type Assert12 = IfSameOutput<(), Applicative<Just<AddOneFunctor>, Nothing>, Nothing>;
    type Assert13 = IfSameOutput<(), Applicative<Nothing, Just<U7>>, Nothing>;
    type Assert14 = IfSameOutput<(), Applicative<Just<AddOneFunctor>, Just<U7>>, Just<U8>>;

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
        let _: Assert9 = ();
        let _: Assert10 = ();
        let _: Assert11 = ();
        let _: Assert12 = ();
        let _: Assert13 = ();
        let _: Assert14 = ();
    }
}
