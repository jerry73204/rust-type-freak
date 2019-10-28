use super::ApplyMap;
use std::marker::PhantomData;

/// A [Map](super::Map) that applies input wrapped [Map](super::Map)
/// to `Rhs` contained type.
pub struct ApplicativeMap<Rhs> {
    _phantom: PhantomData<Rhs>,
}

pub type Applicative<Lhs, Rhs> = ApplyMap<ApplicativeMap<Rhs>, Lhs>;
