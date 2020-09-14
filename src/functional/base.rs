// map trait

pub trait Func<Inputs> {
    type Output;
}

// compose maps

pub struct Compose<First, Second> {
    pub lhs: First,
    pub rhs: Second,
}

impl<Inputs, First, Second> Func<Inputs> for Compose<First, Second>
where
    First: Func<Inputs>,
    Second: Func<<First as Func<Inputs>>::Output>,
{
    type Output = <Second as Func<<First as Func<Inputs>>::Output>>::Output;
}
