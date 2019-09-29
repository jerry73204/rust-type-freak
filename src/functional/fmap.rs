use super::ApplyFunctor;
use std::marker::PhantomData;

/// A functor that applies `Func` to input container type.
pub struct FMapFunctor<Func> {
    _phantom: PhantomData<Func>,
}

pub type FMap<Container, Func> = ApplyFunctor<FMapFunctor<Func>, Container>;
