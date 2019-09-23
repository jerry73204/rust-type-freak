use std::{marker::PhantomData, ops::Add};
use typenum::{Sum, Unsigned, U0, U1};

/// A trait that counts the number of steps.
///
/// It is useful for contructing recursive type operators.
/// [Current] can indicate the termination point of recursion,
/// while [Next] indicates a recursion step before termination.
pub trait Counter {}

pub struct Next<Count>
where
    Count: Counter,
{
    _phantom: PhantomData<Count>,
}

impl<Cnt> Counter for Next<Cnt> where Cnt: Counter {}

pub struct Current;

impl Counter for Current {}

// count op

pub trait Count
where
    Self::Out: Unsigned,
{
    type Out;
}

impl Count for Current {
    type Out = U0;
}

impl<Cnt> Count for Next<Cnt>
where
    Cnt: Counter + Count,
    CountOut<Cnt>: Add<U1>,
    Sum<CountOut<Cnt>, U1>: Unsigned,
{
    type Out = Sum<CountOut<Cnt>, U1>;
}

pub type CountOut<Cnt> = <Cnt as Count>::Out;
