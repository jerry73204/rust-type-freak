use super::{
    Irreducible, Reciprocal, UFracAdd, UFracAddOp, UFracDiv, UFracDivOp, UFracMul, UFracMulOp,
    UFracSub, UFracSubOp, UFraction,
};
use crate::{
    common::*,
    control::ops::AssertSame,
    numeric::{Gcd, GcdOp},
};
use typenum::U1;

// unsigned fraction type

pub struct UFrac<Numerators, Denominators>(PhantomData<(Numerators, Denominators)>)
where
    Numerators: Unsigned,
    Denominators: Unsigned + NonZero;

impl<N, D> UFraction for UFrac<N, D>
where
    N: Unsigned,
    D: Unsigned + NonZero,
{
    fn new() -> Self {
        UFrac(PhantomData)
    }
}

// non-zero trait

impl<N, D> NonZero for UFrac<N, D>
where
    N: Unsigned + NonZero,
    D: Unsigned + NonZero,
{
}

impl<N, D> Irreducible for UFrac<N, D>
where
    (): AssertSame<GcdOp<N, D>, U1, ()> + Gcd<N, D>,
    N: Unsigned,
    D: Unsigned + NonZero,
{
}

// sum

impl<N, D, Rhs> Add<Rhs> for UFrac<N, D>
where
    (): UFracAdd<Self, Rhs>,
    N: Unsigned,
    D: Unsigned + NonZero,
    Rhs: UFraction,
{
    type Output = UFracAddOp<Self, Rhs>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

// subtraction of unsigned fractions

impl<N, D, Rhs> Sub<Rhs> for UFrac<N, D>
where
    (): UFracSub<Self, Rhs>,
    N: Unsigned,
    D: Unsigned + NonZero,
    Rhs: UFraction,
{
    type Output = UFracSubOp<Self, Rhs>;

    fn sub(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

// product of unsigned fractions

impl<N, D, Rhs> Mul<Rhs> for UFrac<N, D>
where
    (): UFracMul<Self, Rhs>,
    N: Unsigned,
    D: Unsigned + NonZero,
    Rhs: UFraction,
{
    type Output = UFracMulOp<Self, Rhs>;

    fn mul(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

// division of unsigned fractions

impl<NL, DL, NR, DR> Div<UFrac<NR, DR>> for UFrac<NL, DL>
where
    (): UFracDiv<UFrac<NL, DL>, UFrac<NR, DR>> + Reciprocal<UFrac<NR, DR>>,
    NL: Unsigned,
    DL: Unsigned + NonZero,
    NR: Unsigned + NonZero,
    DR: Unsigned + NonZero,
{
    type Output = UFracDivOp<UFrac<NL, DL>, UFrac<NR, DR>>;

    fn div(self, _rhs: UFrac<NR, DR>) -> Self::Output {
        Self::Output::new()
    }
}
