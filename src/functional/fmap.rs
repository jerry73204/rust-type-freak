use super::ApplyFunctor;
use std::marker::PhantomData;

pub struct FMapFunctor<Func> {
    _phantom: PhantomData<Func>,
}

pub type FMap<Container, Func> = ApplyFunctor<FMapFunctor<Func>, Container>;
