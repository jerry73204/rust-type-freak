use crate::common::*;
use typenum::consts::*;

pub trait Floating {}

pub struct Float<Base, Significant, Exponent>(Base, Significant, Exponent)
where
    Base: Unsigned + NonZero,
    Significant: Integer,
    Exponent: Integer;

impl<Significant, Base, Exponent> Floating for Float<Base, Significant, Exponent>
where
    Base: Unsigned + NonZero,
    Significant: Integer,
    Exponent: Integer,
{
}

pub type Float2<Significant, Exponent> = Float<U2, Significant, Exponent>;
