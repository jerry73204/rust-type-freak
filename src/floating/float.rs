use typenum::{Bit, Integer, NonZero, Unsigned, U2};

pub struct Float<Sign, Significant, Base, Exponent>(Sign, Significant, Base, Exponent)
where
    Sign: Bit,
    Significant: Unsigned,
    Base: Unsigned + NonZero,
    Exponent: Integer;

pub type Float2<Sign, Significant, Exponent> = Float<Sign, Significant, U2, Exponent>;
