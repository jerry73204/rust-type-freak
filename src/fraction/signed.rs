use super::{
    FracAdd, FracAddOp, FracDiv, FracDivOp, FracMul, FracMulOp, FracSub, FracSubOp, Fraction,
    Irreducible, UFraction,
};
use crate::common::*;

// positive fraction type

pub struct PFrac<Frac>(PhantomData<Frac>)
where
    Frac: UFraction;

impl<Frac> Fraction for PFrac<Frac>
where
    Frac: UFraction,
{
    fn new() -> Self {
        PFrac(PhantomData)
    }
}

// negative fraction type

pub struct NFrac<Frac>(PhantomData<Frac>)
where
    Frac: UFraction;

impl<Frac> Fraction for NFrac<Frac>
where
    Frac: UFraction,
{
    fn new() -> Self {
        NFrac(PhantomData)
    }
}

// non-zero marker

impl<Frac> NonZero for PFrac<Frac> where Frac: UFraction + NonZero {}

impl<Frac> NonZero for NFrac<Frac> where Frac: UFraction + NonZero {}

// irreducible marker

impl<F> Irreducible for PFrac<F> where F: UFraction + Irreducible {}

impl<F> Irreducible for NFrac<F> where F: UFraction + Irreducible {}

// negation

impl<Frac> Neg for PFrac<Frac>
where
    Frac: UFraction,
{
    type Output = NFrac<Frac>;

    fn neg(self) -> Self::Output {
        Self::Output::new()
    }
}

impl<Frac> Neg for NFrac<Frac>
where
    Frac: UFraction,
{
    type Output = PFrac<Frac>;

    fn neg(self) -> Self::Output {
        Self::Output::new()
    }
}

// sum of singed fractions

impl<F, Rhs> Add<Rhs> for PFrac<F>
where
    (): FracAdd<Self, Rhs>,
    F: UFraction,
    Rhs: Fraction,
{
    type Output = FracAddOp<Self, Rhs>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

impl<F, Rhs> Add<Rhs> for NFrac<F>
where
    (): FracAdd<Self, Rhs>,
    F: UFraction,
    Rhs: Fraction,
{
    type Output = FracAddOp<Self, Rhs>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

// subtraction of signed fractions

impl<F, Rhs> Sub<Rhs> for PFrac<F>
where
    (): FracSub<Self, Rhs>,
    F: UFraction,
    Rhs: Fraction,
{
    type Output = FracSubOp<Self, Rhs>;

    fn sub(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

impl<F, Rhs> Sub<Rhs> for NFrac<F>
where
    (): FracSub<Self, Rhs>,
    F: UFraction,
    Rhs: Fraction,
{
    type Output = FracSubOp<Self, Rhs>;

    fn sub(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

// product

impl<F, Rhs> Mul<Rhs> for PFrac<F>
where
    (): FracMul<Self, Rhs>,
    F: UFraction,
    Rhs: Fraction,
{
    type Output = FracMulOp<Self, Rhs>;

    fn mul(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

impl<F, Rhs> Mul<Rhs> for NFrac<F>
where
    (): FracMul<Self, Rhs>,
    F: UFraction,
    Rhs: Fraction,
{
    type Output = FracMulOp<Self, Rhs>;

    fn mul(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

// division

impl<F, Rhs> Div<Rhs> for PFrac<F>
where
    (): FracDiv<Self, Rhs>,
    F: UFraction,
    Rhs: Fraction,
{
    type Output = FracDivOp<Self, Rhs>;

    fn div(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}

impl<F, Rhs> Div<Rhs> for NFrac<F>
where
    (): FracDiv<Self, Rhs>,
    F: UFraction,
    Rhs: Fraction,
{
    type Output = FracDivOp<Self, Rhs>;

    fn div(self, _rhs: Rhs) -> Self::Output {
        Self::Output::new()
    }
}
