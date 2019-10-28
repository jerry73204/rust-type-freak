//! Type operators for tuple types.

use crate::functional::{ApplyMap, Map};

// marker traits for tuples

/// Represents an empty tuple.
pub trait EmptyTuple {}

impl EmptyTuple for () {}

/// Represents a tuple with single type.
pub trait SingleTuple {}

impl<A> SingleTuple for (A,) {}

/// Represents a tuple with double types.
pub trait Pair {}

impl<A, B> Pair for (A, B) {}

/// Represents a tuple with three types.
pub trait Triple {}

impl<A, B, C> Triple for (A, B, C) {}

/// Represents a tuple with four types.
pub trait Quadruple {}

impl<A, B, C, D> Quadruple for (A, B, C, D) {}

// first type of pair

/// A [Map] that takes first type of tuple.
pub struct FirstOfMap {}

pub type FirstOf<Tuple> = ApplyMap<FirstOfMap, Tuple>;

impl<A> Map<(A,)> for FirstOfMap {
    type Output = A;
}

impl<A, B> Map<(A, B)> for FirstOfMap {
    type Output = A;
}

impl<A, B, C> Map<(A, B, C)> for FirstOfMap {
    type Output = A;
}

impl<A, B, C, D> Map<(A, B, C, D)> for FirstOfMap {
    type Output = A;
}

impl<A, B, C, D, E> Map<(A, B, C, D, E)> for FirstOfMap {
    type Output = A;
}

// second type of pair

/// A [Map] that takes second type of tuple.
pub struct SecondOfMap {}

pub type SecondOf<Tuple> = ApplyMap<SecondOfMap, Tuple>;

impl<A, B> Map<(A, B)> for SecondOfMap {
    type Output = B;
}

impl<A, B, C> Map<(A, B, C)> for SecondOfMap {
    type Output = B;
}

impl<A, B, C, D> Map<(A, B, C, D)> for SecondOfMap {
    type Output = B;
}

impl<A, B, C, D, E> Map<(A, B, C, D, E)> for SecondOfMap {
    type Output = B;
}

// thirt type of pair

/// A [Map] that takes third type of tuple.
pub struct ThirdOfMap {}

pub type ThirdOf<Tuple> = ApplyMap<ThirdOfMap, Tuple>;

impl<A, B, C> Map<(A, B, C)> for ThirdOfMap {
    type Output = C;
}

impl<A, B, C, D> Map<(A, B, C, D)> for ThirdOfMap {
    type Output = C;
}

impl<A, B, C, D, E> Map<(A, B, C, D, E)> for ThirdOfMap {
    type Output = C;
}

// left associate

/// A [Map] that transforms `(A, (B, C))` type to `((A, B), C)`.
pub struct LeftAssociateMap {}

pub type LeftAssociate<Tuple> = ApplyMap<LeftAssociateMap, Tuple>;

impl<A, B, C> Map<(A, (B, C))> for LeftAssociateMap {
    type Output = ((A, B), C);
}

// Right associate

/// A [Map] that transforms `((A, B), C)` type to `(A, (B, C))`.
pub struct RightAssociateMap {}

pub type RightAssociate<Tuple> = ApplyMap<RightAssociateMap, Tuple>;

impl<A, B, C> Map<((A, B), C)> for RightAssociateMap {
    type Output = (A, (B, C));
}
