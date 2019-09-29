use super::ApplyFunctor;
use std::marker::PhantomData;

/// A [Functor](super::Functor) that applies input wrapped [Functor](super::Functor)
/// to `Rhs` contained type.
pub struct ApplicativeFunctor<Rhs> {
    _phantom: PhantomData<Rhs>,
}

pub type Applicative<Lhs, Rhs> = ApplyFunctor<ApplicativeFunctor<Rhs>, Lhs>;
