//! Numeric type operators and functors.

use crate::{
    boolean::Boolean,
    control::{IfElsePredicate, IfElsePredicateOutput},
    functional::Functor,
};
use std::ops::{Add, Mul, Sub};
use typenum::{Add1, Gr, IsGreater, IsLess, Le, Prod, Sub1, Sum, B1};

/// A [Functor] type that computes summation of inputs.
pub struct SumComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for SumComposeFunctor
where
    Lhs: Add<Rhs>,
{
    type Output = Sum<Lhs, Rhs>;
}

/// A [Functor] type that computes product of inputs.
pub struct ProdComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for ProdComposeFunctor
where
    Lhs: Mul<Rhs>,
{
    type Output = Prod<Lhs, Rhs>;
}

/// A [Functor] type that gets minimum of inputs.
pub struct MinComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for MinComposeFunctor
where
    Rhs: IsLess<Lhs> + IfElsePredicate<Le<Rhs, Lhs>, Lhs>,
    Le<Rhs, Lhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Rhs, Le<Rhs, Lhs>, Lhs>;
}

/// A [Functor] type that gets maximum of inputs.
pub struct MaxComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for MaxComposeFunctor
where
    Rhs: IsGreater<Lhs> + IfElsePredicate<Gr<Rhs, Lhs>, Lhs>,
    Gr<Rhs, Lhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Rhs, Gr<Rhs, Lhs>, Lhs>;
}

/// A [Functor] that increases input [typenum] integer by one.
pub struct AddOneFunctor;

impl<Value> Functor<Value> for AddOneFunctor
where
    Value: Add<B1>,
{
    type Output = Add1<Value>;
}

/// A [Functor] that decreases input [typenum] integer by one.
pub struct SubOneFunctor;

impl<Value> Functor<Value> for SubOneFunctor
where
    Value: Sub<B1>,
{
    type Output = Sub1<Value>;
}
