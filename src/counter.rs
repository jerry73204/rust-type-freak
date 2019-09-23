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
    Self::Output: Unsigned,
{
    type Output;
}

impl Count for Current {
    type Output = U0;
}

impl<Cnt> Count for Next<Cnt>
where
    Cnt: Counter + Count,
    CountOutput<Cnt>: Add<U1>,
    Sum<CountOutput<Cnt>, U1>: Unsigned,
{
    type Output = Sum<CountOutput<Cnt>, U1>;
}

pub type CountOutput<Cnt> = <Cnt as Count>::Output;
