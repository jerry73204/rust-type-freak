//! Numeric type operators and functors.

use crate::{
    boolean::Boolean,
    control::{IfElsePredicate, IfElsePredicateOutput},
    functional::Map,
};
use std::ops::{Add, Mul, Sub};
use typenum::{Add1, Gr, IsGreater, IsLess, Le, Prod, Sub1, Sum, B1};

/// A [Map] type that computes summation of inputs.
pub struct SumComposeMap;

impl<Lhs, Rhs> Map<(Lhs, Rhs)> for SumComposeMap
where
    Lhs: Add<Rhs>,
{
    type Output = Sum<Lhs, Rhs>;
}

/// A [Map] type that computes product of inputs.
pub struct ProdComposeMap;

impl<Lhs, Rhs> Map<(Lhs, Rhs)> for ProdComposeMap
where
    Lhs: Mul<Rhs>,
{
    type Output = Prod<Lhs, Rhs>;
}

/// A [Map] type that gets minimum of inputs.
pub struct MinComposeMap;

impl<Lhs, Rhs> Map<(Lhs, Rhs)> for MinComposeMap
where
    Rhs: IsLess<Lhs> + IfElsePredicate<Le<Rhs, Lhs>, Lhs>,
    Le<Rhs, Lhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Rhs, Le<Rhs, Lhs>, Lhs>;
}

/// A [Map] type that gets maximum of inputs.
pub struct MaxComposeMap;

impl<Lhs, Rhs> Map<(Lhs, Rhs)> for MaxComposeMap
where
    Rhs: IsGreater<Lhs> + IfElsePredicate<Gr<Rhs, Lhs>, Lhs>,
    Gr<Rhs, Lhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Rhs, Gr<Rhs, Lhs>, Lhs>;
}

/// A [Map] that increases input [typenum] integer by one.
pub struct AddOneMap;

impl<Value> Map<Value> for AddOneMap
where
    Value: Add<B1>,
{
    type Output = Add1<Value>;
}

/// A [Map] that decreases input [typenum] integer by one.
pub struct SubOneMap;

impl<Value> Map<Value> for SubOneMap
where
    Value: Sub<B1>,
{
    type Output = Sub1<Value>;
}
