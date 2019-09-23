use crate::{boolean::Boolean, tuple::SecondOfOut};
use typenum::{
    Eq, False, Gr, GrEq, IsEqual, IsGreater, IsGreaterOrEqual, IsLess, IsLessOrEqual, Le, LeEq,
    True,
};

// if

/// A type operator that checks if type can be constructed.
pub trait If<Cond> {
    type Out;
}

pub type IfOut<Out, Cond> = <Out as If<Cond>>::Out;

impl<Cond, Output> If<Cond> for Output {
    type Out = SecondOfOut<(Cond, Output)>;
}

// if predicate

/// A type operator that checks if condition is [True](crate::boolean::True).
pub trait IfPredicate<Cond>
where
    Cond: Boolean,
{
    type Out;
}

pub type IfPredicateOut<Out, Cond> = <Out as IfPredicate<Cond>>::Out;

impl<Output> IfPredicate<True> for Output {
    type Out = Output;
}

// if-else predicate

/// A type operator that returns output depending [Boolean](crate::boolean::Boolean) condition.
pub trait IfElsePredicate<Cond>
where
    Cond: Boolean,
{
    type Out;
}

pub type IfElsePredicateOut<TrueOut, FalseOut, Cond> =
    <(TrueOut, FalseOut) as IfElsePredicate<Cond>>::Out;

impl<TrueOut, FalseOut> IfElsePredicate<True> for (TrueOut, FalseOut) {
    type Out = TrueOut;
}

impl<TrueOut, FalseOut> IfElsePredicate<False> for (TrueOut, FalseOut) {
    type Out = FalseOut;
}

// if not predicate

/// A type operator that checks if condition is [False](crate::boolean::False).
pub trait IfNotPredicate<Cond>
where
    Cond: Boolean,
{
    type Out;
}

pub type IfNotPredicateOut<Out, Cond> = <Out as IfPredicate<Cond>>::Out;

impl<Output> IfNotPredicate<False> for Output {
    type Out = Output;
}

// if less than

/// A type operator that checks if left-hand-site is less than right-hand-side.
pub trait IfLess<Lhs, Rhs> {
    type Out;
}

pub type IfLessOut<Out, Lhs, Rhs> = <Out as IfLess<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfLess<Lhs, Rhs> for Output
where
    Lhs: IsLess<Rhs>,
    Output: IfPredicate<Le<Lhs, Rhs>>,
    Le<Lhs, Rhs>: Boolean,
{
    type Out = IfPredicateOut<Output, Le<Lhs, Rhs>>;
}

// if less than or equal

/// A type operator that checks if left-hand-site is less than or equals to right-hand-side.
pub trait IfLessOrEqual<Lhs, Rhs> {
    type Out;
}

pub type IfLessOrEqualOut<Out, Lhs, Rhs> = <Out as IfLessOrEqual<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfLessOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsLessOrEqual<Rhs>,
    Output: IfPredicate<LeEq<Lhs, Rhs>>,
    LeEq<Lhs, Rhs>: Boolean,
{
    type Out = IfPredicateOut<Output, LeEq<Lhs, Rhs>>;
}

// if greater than

/// A type operator that checks if left-hand-site is greater than right-hand-side.
pub trait IfGreater<Lhs, Rhs> {
    type Out;
}

pub type IfGreaterOut<Out, Lhs, Rhs> = <Out as IfGreater<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfGreater<Lhs, Rhs> for Output
where
    Lhs: IsGreater<Rhs>,
    Output: IfPredicate<Gr<Lhs, Rhs>>,
    Gr<Lhs, Rhs>: Boolean,
{
    type Out = IfPredicateOut<Output, Gr<Lhs, Rhs>>;
}

// if greater than or equal

/// A type operator that checks if left-hand-site is greater than or equals to right-hand-side.
pub trait IfGreaterOrEqual<Lhs, Rhs> {
    type Out;
}

pub type IfGreaterOrEqualOut<Out, Lhs, Rhs> = <Out as IfGreaterOrEqual<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfGreaterOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsGreaterOrEqual<Rhs>,
    Output: IfPredicate<GrEq<Lhs, Rhs>>,
    GrEq<Lhs, Rhs>: Boolean,
{
    type Out = IfPredicateOut<Output, GrEq<Lhs, Rhs>>;
}

// if equal

/// A type operator that checks if left-hand-site equals to right-hand-side.
pub trait IfEqual<Lhs, Rhs> {
    type Out;
}

pub type IfEqualOut<Out, Lhs, Rhs> = <Out as IfEqual<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfEqual<Lhs, Rhs> for Output
where
    Lhs: IsEqual<Rhs>,
    Output: IfPredicate<Eq<Lhs, Rhs>>,
    Eq<Lhs, Rhs>: Boolean,
{
    type Out = IfPredicateOut<Output, Eq<Lhs, Rhs>>;
}
