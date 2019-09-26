//! Type operators for tuple types.

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

/// A type operator that takes first type of tuple.
pub trait FirstOf {
    type Output;
}

pub type FirstOfOutput<T> = <T as FirstOf>::Output;

impl<A> FirstOf for (A,) {
    type Output = A;
}

impl<A, B> FirstOf for (A, B) {
    type Output = A;
}

impl<A, B, C> FirstOf for (A, B, C) {
    type Output = A;
}

impl<A, B, C, D> FirstOf for (A, B, C, D) {
    type Output = A;
}

impl<A, B, C, D, E> FirstOf for (A, B, C, D, E) {
    type Output = A;
}

// second type of pair

/// A type operator that takes second type of tuple.
pub trait SecondOf {
    type Output;
}

pub type SecondOfOutput<T> = <T as SecondOf>::Output;

impl<A, B> SecondOf for (A, B) {
    type Output = B;
}

impl<A, B, C> SecondOf for (A, B, C) {
    type Output = B;
}

impl<A, B, C, D> SecondOf for (A, B, C, D) {
    type Output = B;
}

impl<A, B, C, D, E> SecondOf for (A, B, C, D, E) {
    type Output = B;
}

// thirt type of pair

/// A type operator that takes third type of tuple.
pub trait ThirdOf {
    type Output;
}

pub type ThirdOfOutput<T> = <T as ThirdOf>::Output;

impl<A, B, C> ThirdOf for (A, B, C) {
    type Output = C;
}

impl<A, B, C, D> ThirdOf for (A, B, C, D) {
    type Output = C;
}

impl<A, B, C, D, E> ThirdOf for (A, B, C, D, E) {
    type Output = C;
}

// left associate

pub trait LeftAssociate {
    type Output;
}

pub type LeftAssociateOutput<T> = <T as LeftAssociate>::Output;

impl<A, B, C> LeftAssociate for (A, (B, C)) {
    type Output = ((A, B), C);
}

// Right associate

pub trait RightAssociate {
    type Output;
}

pub type RightAssociateOutput<T> = <T as RightAssociate>::Output;

impl<A, B, C> RightAssociate for ((A, B), C) {
    type Output = (A, (B, C));
}
