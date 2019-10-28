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

use crate::functional::{ApplicativeMap, ApplyMap, FMapMap, Map};
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

/// A [Map] that unwraps [Just<T>](Just).
pub struct UnwrapMap {}

pub type Unwrap<T> = ApplyMap<UnwrapMap, T>;

impl<T> Map<Just<T>> for UnwrapMap {
    type Output = T;
}

// unwrap or default op

/// A [Map] that unwraps [Just<T>](Just), or returns `Defaultvalue` if [Nothing].
pub struct UnwrapOrMap<DefaultValue> {
    _phantom: PhantomData<DefaultValue>,
}

pub type UnwrapOr<T, DefaultValue> = ApplyMap<UnwrapOrMap<DefaultValue>, T>;

impl<T, DefaultValue> Map<Just<T>> for UnwrapOrMap<DefaultValue> {
    type Output = T;
}

impl<DefaultValue> Map<Nothing> for UnwrapOrMap<DefaultValue> {
    type Output = DefaultValue;
}

// map the value of maybe

/// A [Map] that maps a [Maybe] type by a [Map].
pub struct MaybeMapMap<Func> {
    _phantom: PhantomData<Func>,
}

pub type MaybeMap<Input, Func> = ApplyMap<MaybeMapMap<Func>, Input>;

impl<Func> Map<Nothing> for MaybeMapMap<Func> {
    type Output = Nothing;
}

impl<Func, T> Map<Just<T>> for MaybeMapMap<Func>
where
    Func: Map<T>,
{
    type Output = Just<ApplyMap<Func, T>>;
}

// filter

/// A [Map] that filters a [Maybe] type by a [Map].
pub struct MaybeFilterMap<Func> {
    _phantom: PhantomData<Func>,
}

pub type MaybeFilter<Input, Func> = ApplyMap<MaybeFilterMap<Func>, Input>;

impl<Func> Map<Nothing> for MaybeFilterMap<Func> {
    type Output = Nothing;
}

impl<Func, T> Map<Just<T>> for MaybeFilterMap<Func>
where
    Func: Map<T>,
    Func::Output: Maybe,
{
    type Output = ApplyMap<Func, T>;
}

// impl FMap for Maybe

impl<Func> Map<Nothing> for FMapMap<Func> {
    type Output = Nothing;
}

impl<T, Func> Map<Just<T>> for FMapMap<Func>
where
    MaybeMapMap<Func>: Map<Just<T>>,
{
    type Output = MaybeMap<Just<T>, Func>;
}

// Applicative

impl Map<Nothing> for ApplicativeMap<Nothing> {
    type Output = Nothing;
}

impl<Func> Map<Just<Func>> for ApplicativeMap<Nothing> {
    type Output = Nothing;
}

impl<Value> Map<Nothing> for ApplicativeMap<Just<Value>> {
    type Output = Nothing;
}

impl<Func, Value> Map<Just<Func>> for ApplicativeMap<Just<Value>>
where
    Func: Map<Value>,
{
    type Output = Just<ApplyMap<Func, Value>>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        boolean::Boolean,
        control::{IfElsePredicate, IfElsePredicateOutput, IfSameOutput},
        functional::{Applicative, FMap},
        numeric::AddOneMap,
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

    impl<Input> Map<Input> for BoxFunc {
        type Output = Box<Input>;
    }

    type Assert4 = IfSameOutput<(), MaybeMap<Just<i8>, BoxFunc>, Just<Box<i8>>>;
    type Assert5 = IfSameOutput<(), MaybeMap<Nothing, BoxFunc>, Nothing>;

    // filter
    struct ThresholdFunc;

    impl<Input> Map<Input> for ThresholdFunc
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
    type Assert9 = IfSameOutput<(), FMap<Nothing, AddOneMap>, Nothing>;
    type Assert10 = IfSameOutput<(), FMap<Just<U8>, AddOneMap>, Just<U9>>;

    // Applicative
    type Assert11 = IfSameOutput<(), Applicative<Nothing, Nothing>, Nothing>;
    type Assert12 = IfSameOutput<(), Applicative<Just<AddOneMap>, Nothing>, Nothing>;
    type Assert13 = IfSameOutput<(), Applicative<Nothing, Just<U7>>, Nothing>;
    type Assert14 = IfSameOutput<(), Applicative<Just<AddOneMap>, Just<U7>>, Just<U8>>;

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
