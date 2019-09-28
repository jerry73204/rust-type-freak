use super::ApplyFunctor;
use std::marker::PhantomData;

pub struct ApplicativeFunctor<Rhs> {
    _phantom: PhantomData<Rhs>,
}

pub type Applicative<Lhs, Rhs> = ApplyFunctor<ApplicativeFunctor<Rhs>, Lhs>;
