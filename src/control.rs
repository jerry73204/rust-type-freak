use crate::{
    boolean::{Boolean, False, True},
    tuple::SecondOfOut,
};
use typenum::{IsEqual, IsGreater, IsGreaterOrEqual, IsLess, IsLessOrEqual};

// if

/// A trait operator that checks if type can be constructed.
pub trait If<Cond> {
    type Out;
}

pub type IfOut<Out, Cond> = <Out as If<Cond>>::Out;

impl<Cond, Output> If<Cond> for Output {
    type Out = SecondOfOut<(Cond, Output)>;
}

// if predicate

/// A trait operator that checks if condition is [True](crate::boolean::True).
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

/// A trait operator that returns output depending [Boolean](crate::boolean::Boolean) condition.
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

/// A trait operator that checks if condition is [False](crate::boolean::False).
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

/// A trait operator that checks if left-hand-site is less than right-hand-side.
pub trait IfLess<Lhs, Rhs>
where
    Lhs: IsLess<Rhs>,
{
    type Out;
}

pub type IfLessOut<Out, Lhs, Rhs> = <Out as IfLess<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfLess<Lhs, Rhs> for Output
where
    Lhs: IsLess<Rhs>,
{
    type Out = Output;
}

// if less than or equal

/// A trait operator that checks if left-hand-site is less than or equals to right-hand-side.
pub trait IfLessOrEqual<Lhs, Rhs>
where
    Lhs: IsLessOrEqual<Rhs>,
{
    type Out;
}

pub type IfLessOrEqualOut<Out, Lhs, Rhs> = <Out as IfLessOrEqual<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfLessOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsLessOrEqual<Rhs>,
{
    type Out = Output;
}

// if greater than

/// A trait operator that checks if left-hand-site is greater than right-hand-side.
pub trait IfGreater<Lhs, Rhs>
where
    Lhs: IsGreater<Rhs>,
{
    type Out;
}

pub type IfGreaterOut<Out, Lhs, Rhs> = <Out as IfGreater<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfGreater<Lhs, Rhs> for Output
where
    Lhs: IsGreater<Rhs>,
{
    type Out = Output;
}

// if greater than or equal

/// A trait operator that checks if left-hand-site is greater than or equals to right-hand-side.
pub trait IfGreaterOrEqual<Lhs, Rhs>
where
    Lhs: IsGreaterOrEqual<Rhs>,
{
    type Out;
}

pub type IfGreaterOrEqualOut<Out, Lhs, Rhs> = <Out as IfGreaterOrEqual<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfGreaterOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsGreaterOrEqual<Rhs>,
{
    type Out = Output;
}

// if equal

/// A trait operator that checks if left-hand-site equals to right-hand-side.
pub trait IfEqual<Lhs, Rhs>
where
    Lhs: IsEqual<Rhs>,
{
    type Out;
}

pub type IfEqualOut<Out, Lhs, Rhs> = <Out as IfEqual<Lhs, Rhs>>::Out;

impl<Lhs, Rhs, Output> IfEqual<Lhs, Rhs> for Output
where
    Lhs: IsEqual<Rhs>,
{
    type Out = Output;
}
