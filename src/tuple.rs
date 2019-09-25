//! Type operators for tuple types.

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
