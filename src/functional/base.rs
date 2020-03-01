use std::marker::PhantomData;

// map trait

pub trait Map<Inputs> {
    type Output;
}

pub type Apply<MapType, Inputs> = <MapType as Map<Inputs>>::Output;

// compose maps

pub struct Compose<Lhs, Rhs>(PhantomData<(Lhs, Rhs)>);

impl<Inputs, Lhs, Rhs> Map<Inputs> for Compose<Lhs, Rhs>
where
    Lhs: Map<Inputs>,
    Rhs: Map<Apply<Lhs, Inputs>>
{
    type Output = Apply<Rhs, Apply<Lhs, Inputs>>;
}
