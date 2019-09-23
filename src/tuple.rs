// first type of pair

/// A type operator that takes first type of tuple.
pub trait FirstOf {
    type Out;
}

pub type FirstOfOut<T> = <T as FirstOf>::Out;

impl<A> FirstOf for (A,) {
    type Out = A;
}

impl<A, B> FirstOf for (A, B) {
    type Out = A;
}

impl<A, B, C> FirstOf for (A, B, C) {
    type Out = A;
}

impl<A, B, C, D> FirstOf for (A, B, C, D) {
    type Out = A;
}

impl<A, B, C, D, E> FirstOf for (A, B, C, D, E) {
    type Out = A;
}

// second type of pair

/// A type operator that takes second type of tuple.
pub trait SecondOf {
    type Out;
}

pub type SecondOfOut<T> = <T as SecondOf>::Out;

impl<A, B> SecondOf for (A, B) {
    type Out = B;
}

impl<A, B, C> SecondOf for (A, B, C) {
    type Out = B;
}

impl<A, B, C, D> SecondOf for (A, B, C, D) {
    type Out = B;
}

impl<A, B, C, D, E> SecondOf for (A, B, C, D, E) {
    type Out = B;
}

// thirt type of pair

/// A type operator that takes third type of tuple.
pub trait ThirdOf {
    type Out;
}

pub type ThirdOfOut<T> = <T as ThirdOf>::Out;

impl<A, B, C> ThirdOf for (A, B, C) {
    type Out = C;
}

impl<A, B, C, D> ThirdOf for (A, B, C, D) {
    type Out = C;
}

impl<A, B, C, D, E> ThirdOf for (A, B, C, D, E) {
    type Out = C;
}

// left associate

pub trait LeftAssociate {
    type Out;
}

pub type LeftAssociateOut<T> = <T as LeftAssociate>::Out;

impl<A, B, C> LeftAssociate for (A, (B, C)) {
    type Out = ((A, B), C);
}

// Right associate

pub trait RightAssociate {
    type Out;
}

pub type RightAssociateOut<T> = <T as RightAssociate>::Out;

impl<A, B, C> RightAssociate for ((A, B), C) {
    type Out = (A, (B, C));
}
