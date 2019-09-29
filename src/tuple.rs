//! Type operators for tuple types.

use crate::functional::{ApplyFunctor, Functor};

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

/// A [Functor] that takes first type of tuple.
pub struct FirstOfFunctor {}

pub type FirstOf<Tuple> = ApplyFunctor<FirstOfFunctor, Tuple>;

impl<A> Functor<(A,)> for FirstOfFunctor {
    type Output = A;
}

impl<A, B> Functor<(A, B)> for FirstOfFunctor {
    type Output = A;
}

impl<A, B, C> Functor<(A, B, C)> for FirstOfFunctor {
    type Output = A;
}

impl<A, B, C, D> Functor<(A, B, C, D)> for FirstOfFunctor {
    type Output = A;
}

impl<A, B, C, D, E> Functor<(A, B, C, D, E)> for FirstOfFunctor {
    type Output = A;
}

// second type of pair

/// A [Functor] that takes second type of tuple.
pub struct SecondOfFunctor {}

pub type SecondOf<Tuple> = ApplyFunctor<SecondOfFunctor, Tuple>;

impl<A, B> Functor<(A, B)> for SecondOfFunctor {
    type Output = B;
}

impl<A, B, C> Functor<(A, B, C)> for SecondOfFunctor {
    type Output = B;
}

impl<A, B, C, D> Functor<(A, B, C, D)> for SecondOfFunctor {
    type Output = B;
}

impl<A, B, C, D, E> Functor<(A, B, C, D, E)> for SecondOfFunctor {
    type Output = B;
}

// thirt type of pair

/// A [Functor] that takes third type of tuple.
pub struct ThirdOfFunctor {}

pub type ThirdOf<Tuple> = ApplyFunctor<ThirdOfFunctor, Tuple>;

impl<A, B, C> Functor<(A, B, C)> for ThirdOfFunctor {
    type Output = C;
}

impl<A, B, C, D> Functor<(A, B, C, D)> for ThirdOfFunctor {
    type Output = C;
}

impl<A, B, C, D, E> Functor<(A, B, C, D, E)> for ThirdOfFunctor {
    type Output = C;
}

// left associate

/// A [Functor] that transforms `(A, (B, C))` type to `((A, B), C)`.
pub struct LeftAssociateFunctor {}

pub type LeftAssociate<Tuple> = ApplyFunctor<LeftAssociateFunctor, Tuple>;

impl<A, B, C> Functor<(A, (B, C))> for LeftAssociateFunctor {
    type Output = ((A, B), C);
}

// Right associate

/// A [Functor] that transforms `((A, B), C)` type to `(A, (B, C))`.
pub struct RightAssociateFunctor {}

pub type RightAssociate<Tuple> = ApplyFunctor<RightAssociateFunctor, Tuple>;

impl<A, B, C> Functor<((A, B), C)> for RightAssociateFunctor {
    type Output = (A, (B, C));
}
