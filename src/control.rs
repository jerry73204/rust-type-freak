//! Compile-time guards and static assertions.
//!
//! ## Overview
//! Most type operators in this module copies the input type to `Self::Output`
//! when certain conditions are met. We have these categories of operators:
//!
//! - [If<Output, Type>](crate::control::IfOutput):
//!     Asserts `Type` can be constructed.
//! - [IfPredicate<Output, Predicate>](crate::control::IfPredicate):
//!     Asserts `Predicate` derives to [True](typenum::True).
//! - [IfSame<Output, Lhs, Rhs>](crate::control::IfSame):
//!     Asserts `Lhs` and `Rhs` are of the same type.
//! - [IfLess](crate::control::IfLess), [IfLessOrEqual](crate::control::IfLessOrEqual),
//!     [IfGreater](crate::control::IfGreater), [IfGreaterOrEqual](crate::control::IfGreaterOrEqual),
//!     [IfEqual](crate::control::IfEqual):
//!     Asserts two [typenum] numbers follows the order.
//!
//! By convention, [IfSameOutput<Output, Lhs, Rhs>](crate::control::IfSameOutput) is type alias of
//! `<Output as IfSame<Lhs, Rhs>>::Output` trait cast, and others follow.
//! Only [IfOutput<Output, Type>](crate::control::IfOutput) has no corresponding trait.
//!
//! ## Static assertions
//! We can make use of `If*Output` aliases to build compile time assertions.
//! For example, [IfLessOutput](crate::control::IfLessOutput) asserts LHS
//! is less than RHS.
//!
//! ```ignore
//! use typenum::consts::*;
//! use type_freak::control::IfLessOutput;
//!
//! type Out1 = IfLessOutput<usize, U3, U5>;  // U3 < U5 is true, thus Out1 ~= usize
//! type Out2 = IfLessOutput<usize, U5, U3>;  // U5 < U5 is false
//!
//! fn assert() {
//!    let _: Out1 = 0;  // Goes fine here.
//!    let _: Out2 = 0;  // Compile error!!!
//! }
//!  ```
//!
//! ## Compile-time guards
//! By placing `If*` trait bounds in `where` block. we can build compile-time
//! guarded functions. For example, we add `IfSame` trait bound to assert two function
//! generic parameters have identical types.
//! The same trick applies to guarded structs, traits and impl blocks.
//!
//! ```ignore
//! use type_freak::control::IfSame;
//!
//! fn guarded_function<Lhs, Rhs>() -> String
//! where
//!     Lhs: IfSame<Lhs, Rhs>
//! {
//!     "Yeeeeeee!".to_owned()
//! }
//!
//! fn comile_me() {
//!     let _ = guarded_function::<String, String>();  // fine
//!     let _ = guarded_function::<String, u8>();      // Compile error!!!
//! }
//! ```

use crate::{boolean::Boolean, tuple::FirstOf};
use typenum::{
    Eq, False, Gr, GrEq, IsEqual, IsGreater, IsGreaterOrEqual, IsLess, IsLessOrEqual, Le, LeEq,
    NonZero, True, B0, U0, Z0,
};

// if

/// Returns input type if `Cond` can be constructed.
pub trait If<Cond> {
    type Output;
}

pub type IfOutput<Output, Cond> = <Output as If<Cond>>::Output;

impl<Cond, Output> If<Cond> for Output {
    type Output = FirstOf<(Output, Cond)>;
}

// if type equivalence

/// Returns input type if both `Lhs` and `Rhs` are equivalent types.
pub trait IfSame<Lhs, Rhs> {
    type Output;
}

pub type IfSameOutput<Output, Lhs, Rhs> = <Output as IfSame<Lhs, Rhs>>::Output;

impl<Same, Output> IfSame<Same, Same> for Output {
    type Output = Output;
}

// if predicate holds

/// Returns input type if `Cond` evaluates to [True].
pub trait IfPredicate<Cond>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfPredicateOutput<Output, Cond> = <Output as IfPredicate<Cond>>::Output;

impl<Output> IfPredicate<True> for Output {
    type Output = Output;
}

// if predicate is false

/// Returns input type if `Cond` evaluates to [False].
pub trait IfNotPredicate<Cond>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfNotPredicateOutput<Output, Cond> = <Output as IfNotPredicate<Cond>>::Output;

impl<Output> IfNotPredicate<False> for Output {
    type Output = Output;
}

// if-else predicate

/// Returns input type if `Cond` evaluates to [True], otherwise returns `Else`.
pub trait IfElsePredicate<Cond, ElseOutput>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfElsePredicateOutput<TrueOutput, Cond, FalseOutput> =
    <TrueOutput as IfElsePredicate<Cond, FalseOutput>>::Output;

impl<TrueOutput, FalseOutput> IfElsePredicate<True, FalseOutput> for TrueOutput {
    type Output = TrueOutput;
}

impl<TrueOutput, FalseOutput> IfElsePredicate<False, FalseOutput> for TrueOutput {
    type Output = FalseOutput;
}

// if less than

/// Returns input type if `Lhs` is less than `Rhs`.
pub trait IfLess<Lhs, Rhs> {
    type Output;
}

pub type IfLessOutput<Output, Lhs, Rhs> = <Output as IfLess<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfLess<Lhs, Rhs> for Output
where
    Lhs: IsLess<Rhs>,
    Output: IfPredicate<Le<Lhs, Rhs>>,
    Le<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Le<Lhs, Rhs>>;
}

// if-else less than

/// Returns input type if `Lhs` is less than `Rhs`, otherwise returns `Else`.
pub trait IfElseLess<Lhs, Rhs, Else> {
    type Output;
}

pub type IfElseLessOutput<Output, Lhs, Rhs, Else> = <Output as IfElseLess<Lhs, Rhs, Else>>::Output;

impl<Lhs, Rhs, Output, Else> IfElseLess<Lhs, Rhs, Else> for Output
where
    Lhs: IsLess<Rhs>,
    Output: IfElsePredicate<Le<Lhs, Rhs>, Else>,
    Le<Lhs, Rhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Output, Le<Lhs, Rhs>, Else>;
}

// if less than or equal

/// Returns input type if `Lhs` is less than or equals to `Rhs`.
pub trait IfLessOrEqual<Lhs, Rhs> {
    type Output;
}

pub type IfLessOrEqualOutput<Output, Lhs, Rhs> = <Output as IfLessOrEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfLessOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsLessOrEqual<Rhs>,
    Output: IfPredicate<LeEq<Lhs, Rhs>>,
    LeEq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, LeEq<Lhs, Rhs>>;
}

// if-else less or equal

/// Returns input type if `Lhs` is less than or equals to `Rhs`, otherwise returns `Else`.
pub trait IfElseLessOrEqual<Lhs, Rhs, Else> {
    type Output;
}

pub type IfElseLessOrEqualOutput<Output, Lhs, Rhs, Else> =
    <Output as IfElseLessOrEqual<Lhs, Rhs, Else>>::Output;

impl<Lhs, Rhs, Output, Else> IfElseLessOrEqual<Lhs, Rhs, Else> for Output
where
    Lhs: IsLess<Rhs>,
    Output: IfElsePredicate<Le<Lhs, Rhs>, Else>,
    Le<Lhs, Rhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Output, Le<Lhs, Rhs>, Else>;
}

// if greater than

/// Returns input type if `Lhs` is greater than `Rhs`.
pub trait IfGreater<Lhs, Rhs> {
    type Output;
}

pub type IfGreaterOutput<Output, Lhs, Rhs> = <Output as IfGreater<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfGreater<Lhs, Rhs> for Output
where
    Lhs: IsGreater<Rhs>,
    Output: IfPredicate<Gr<Lhs, Rhs>>,
    Gr<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Gr<Lhs, Rhs>>;
}

// if-else greater than

/// Returns input type if `Lhs` is greater than `Rhs`, otherwise returns `Else`.
pub trait IfElseGreater<Lhs, Rhs, Else> {
    type Output;
}

pub type IfElseGreaterOutput<Output, Lhs, Rhs, Else> =
    <Output as IfElseGreater<Lhs, Rhs, Else>>::Output;

impl<Lhs, Rhs, Output, Else> IfElseGreater<Lhs, Rhs, Else> for Output
where
    Lhs: IsLess<Rhs>,
    Output: IfElsePredicate<Le<Lhs, Rhs>, Else>,
    Le<Lhs, Rhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Output, Le<Lhs, Rhs>, Else>;
}

// if greater than or equal

/// Returns input type if `Lhs` is greater than or equals to `Rhs`.
pub trait IfGreaterOrEqual<Lhs, Rhs> {
    type Output;
}

pub type IfGreaterOrEqualOutput<Output, Lhs, Rhs> = <Output as IfGreaterOrEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfGreaterOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsGreaterOrEqual<Rhs>,
    Output: IfPredicate<GrEq<Lhs, Rhs>>,
    GrEq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, GrEq<Lhs, Rhs>>;
}

// if-else greater or equal

/// Returns input type if `Lhs` is greater than or equals to `Rhs`, otherwise returns `Else`.
pub trait IfElseGreaterOrEqual<Lhs, Rhs, Else> {
    type Output;
}

pub type IfElseGreaterOrEqualOutput<Output, Lhs, Rhs, Else> =
    <Output as IfElseGreaterOrEqual<Lhs, Rhs, Else>>::Output;

impl<Lhs, Rhs, Output, Else> IfElseGreaterOrEqual<Lhs, Rhs, Else> for Output
where
    Lhs: IsLess<Rhs>,
    Output: IfElsePredicate<Le<Lhs, Rhs>, Else>,
    Le<Lhs, Rhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Output, Le<Lhs, Rhs>, Else>;
}

// if equal

/// Returns input type if `Lhs` equals to `Rhs`.
pub trait IfEqual<Lhs, Rhs> {
    type Output;
}

pub type IfEqualOutput<Output, Lhs, Rhs> = <Output as IfEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfEqual<Lhs, Rhs> for Output
where
    Lhs: IsEqual<Rhs>,
    Output: IfPredicate<Eq<Lhs, Rhs>>,
    Eq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Eq<Lhs, Rhs>>;
}

// if else equal

/// Returns output if left-hand-site equals to right-hand-side, otherwise returns `Else`.
pub trait IfElseEqual<Lhs, Rhs, Else> {
    type Output;
}

pub type IfElseEqualOutput<Output, Lhs, Rhs, Else> =
    <Output as IfElseEqual<Lhs, Rhs, Else>>::Output;

impl<Lhs, Rhs, Output, Else> IfElseEqual<Lhs, Rhs, Else> for Output
where
    Lhs: IsEqual<Rhs>,
    Output: IfElsePredicate<Eq<Lhs, Rhs>, Else>,
    Eq<Lhs, Rhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Output, Eq<Lhs, Rhs>, Else>;
}

// if zero

/// A type operator that checks if a [typenum] value is either
/// [B0](typenum::B0), [Z0](typenum::Z0) or [U0](typenum::U0).
pub trait IfZero<Value> {
    type Output;
}

pub type IfZeroOutput<Output, Value> = <Output as IfZero<Value>>::Output;

impl<Output> IfZero<B0> for Output {
    type Output = Output;
}

impl<Output> IfZero<Z0> for Output {
    type Output = Output;
}

impl<Output> IfZero<U0> for Output {
    type Output = Output;
}

// if non-zero

/// A type operator that checks if a [typenum] value implements
/// [NonZero](typenum::NonZero) trait.
pub trait IfNonZero<Value>
where
    Value: NonZero,
{
    type Output;
}

pub type IfNonZeroOutput<Output, Value> = <Output as IfNonZero<Value>>::Output;

impl<Value, Output> IfNonZero<Value> for Output
where
    Value: NonZero,
{
    type Output = Output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{consts::*, Le, Unsigned};

    // if constructed
    type Assert1 = IfOutput<U3, ()>;

    // if type equivalence
    type Assert2 = IfSameOutput<(), u8, u8>;

    // if predicate
    type Assert3 = IfPredicateOutput<(), Le<U3, U4>>;

    // if else predicate
    type Assert4 = IfElsePredicateOutput<True, False, Le<U3, U4>>;

    // if less than
    type Assert5 = IfLessOutput<(), U6, U9>;

    // if less than or equal
    type Assert6 = IfLessOrEqualOutput<(), U6, U6>;
    type Assert7 = IfLessOrEqualOutput<(), U6, U7>;

    // if greater than
    type Assert8 = IfGreaterOutput<(), U7, U4>;

    // if greater than or equal
    type Assert9 = IfGreaterOrEqualOutput<(), U7, U4>;
    type Assert10 = IfGreaterOrEqualOutput<(), U7, U7>;

    // if equal
    type Assert11 = IfEqualOutput<(), Z0, Z0>;

    // if zero
    type Assert12<Value> = IfZeroOutput<(), Value>;

    // if non-zero
    type Assert13<Value> = IfNonZeroOutput<(), Value>;

    #[test]
    fn control_test() {
        assert_eq!(3, Assert1::USIZE);
        let _: Assert2 = ();
        let _: Assert3 = ();
        assert!(Assert4::BOOL);
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
        let _: Assert9 = ();
        let _: Assert10 = ();
        let _: Assert11 = ();
        let _: Assert12<B0> = ();
        let _: Assert12<Z0> = ();
        let _: Assert12<U0> = ();
        let _: Assert13<P1> = ();
        let _: Assert13<N1> = ();
        let _: Assert13<U1> = ();
    }
}
