use super::{
    FracAdd, FracAddOp, FracDiv, FracDivOp, FracMul, FracMulOp, FracSub, FracSubOp, Fraction,
    Irreducible, UFracAdd, UFracDiv, UFracMul, UFracSub, UFraction,
};
use crate::{common::*, Frac};

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

// is less

// impl<LF, RF> IsLess<PFrac<RF>> for PFrac<LF>
// where
//     LF: UFraction + IsLess<RF>,
//     RF: UFraction,
// {
//     type Output = Le<LF, RF>;

//     fn is_less(self, _rhs: PFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LF, RF> IsLess<NFrac<RF>> for NFrac<LF>
// where
//     LF: UFraction + IsGreater<RF>,
//     RF: UFraction,
// {
//     type Output = Gr<LF, RF>;

//     fn is_less(self, _rhs: NFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LD, RD> IsLess<Frac!(UTerm, RD)> for Frac!(-UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_less(self, _rhs: Frac!(UTerm, RD)) -> Self::Output {
//         B0
//     }
// }

// impl<LD, RU, RB, RD> IsLess<Frac!(UInt<RU, RB>, RD)> for Frac!(-UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RU: Unsigned,
//     RB: Bit,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_less(self, _rhs: Frac!(UInt<RU, RB>, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LU, LB, LD, RD> IsLess<Frac!(UTerm, RD)> for Frac!(- UInt<LU, LB>, LD)
// where
//     LU: Unsigned,
//     LB: Bit,
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_less(self, _rhs: Frac!(UTerm, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LU, LB, LD, RU, RB, RD> IsLess<Frac!(UInt<RU, RB>, RD)> for Frac!(- UInt<LU, LB>, LD)
// where
//     LU: Unsigned,
//     LB: Bit,
//     LD: Unsigned + NonZero,
//     RU: Unsigned,
//     RB: Bit,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_less(self, _rhs: Frac!(UInt<RU, RB>, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LN, LD, RN, RD> IsLess<Frac!(-RN, RD)> for Frac!(LN, LD)
// where
//     LN: Unsigned,
//     LD: Unsigned + NonZero,
//     RN: Unsigned,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_less(self, _rhs: Frac!(-RN, RD)) -> Self::Output {
//         B0
//     }
// }

// is less or equal

// impl<LF, RF> IsLessOrEqual<PFrac<RF>> for PFrac<LF>
// where
//     LF: UFraction + IsLessOrEqual<RF>,
//     RF: UFraction,
// {
//     type Output = LeEq<LF, RF>;

//     fn is_less_or_equal(self, _rhs: PFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LF, RF> IsLessOrEqual<NFrac<RF>> for NFrac<LF>
// where
//     LF: UFraction + IsGreaterOrEqual<RF>,
//     RF: UFraction,
// {
//     type Output = GrEq<LF, RF>;

//     fn is_less_or_equal(self, _rhs: NFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LD, RD> IsLessOrEqual<Frac!(-UTerm, RD)> for Frac!(UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_less_or_equal(self, _rhs: Frac!(-UTerm, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LD, RU, RB, RD> IsLessOrEqual<Frac!(- UInt<RU, RB>, RD)> for Frac!(UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RU: Unsigned,
//     RB: Bit,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_less_or_equal(self, _rhs: Frac!(- UInt<RU, RB>, RD)) -> Self::Output {
//         B0
//     }
// }

// impl<LU, LB, LD, RD> IsLessOrEqual<Frac!(-UTerm, RD)> for Frac!(UInt<LU, LB>, LD)
// where
//     LU: Unsigned,
//     LB: Bit,
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_less_or_equal(self, _rhs: Frac!(-UTerm, RD)) -> Self::Output {
//         B0
//     }
// }

// impl<LU, LB, LD, RU, RB, RD> IsLessOrEqual<Frac!(- UInt<RU, RB>, RD)> for Frac!(UInt<LU, LB>, LD)
// where
//     LU: Unsigned,
//     LB: Bit,
//     LD: Unsigned + NonZero,
//     RU: Unsigned,
//     RB: Bit,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_less_or_equal(self, _rhs: Frac!(- UInt<RU, RB>, RD)) -> Self::Output {
//         B0
//     }
// }

// impl<LN, LD, RN, RD> IsLessOrEqual<Frac!(RN, RD)> for Frac!(-LN, LD)
// where
//     LN: Unsigned,
//     LD: Unsigned + NonZero,
//     RN: Unsigned,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_less_or_equal(self, _rhs: Frac!(RN, RD)) -> Self::Output {
//         B1
//     }
// }

// // is greater

// impl<LF, RF> IsGreater<PFrac<RF>> for PFrac<LF>
// where
//     LF: UFraction + IsGreater<RF>,
//     RF: UFraction,
// {
//     type Output = Gr<LF, RF>;

//     fn is_greater(self, _rhs: PFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LF, RF> IsGreater<NFrac<RF>> for NFrac<LF>
// where
//     LF: UFraction + IsLess<RF>,
//     RF: UFraction,
// {
//     type Output = Le<LF, RF>;

//     fn is_greater(self, _rhs: NFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LD, RD> IsGreater<Frac!(-UTerm, RD)> for Frac!(UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_greater(self, _rhs: Frac!(-UTerm, RD)) -> Self::Output {
//         B0
//     }
// }

// impl<LD, RU, RB, RD> IsGreater<Frac!(- UInt<RU, RB>, RD)> for Frac!(UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RU: Unsigned,
//     RB: Bit,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_greater(self, _rhs: Frac!(- UInt<RU, RB>, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LU, LB, LD, RD> IsGreater<Frac!(-UTerm, RD)> for Frac!(UInt<LU, LB>, LD)
// where
//     LU: Unsigned,
//     LB: Bit,
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_greater(self, _rhs: Frac!(-UTerm, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LU, LB, LD, RU, RB, RD> IsGreater<Frac!(- UInt<RU, RB>, RD)> for Frac!(UInt<LU, LB>, LD)
// where
//     LU: Unsigned,
//     LB: Bit,
//     LD: Unsigned + NonZero,
//     RU: Unsigned,
//     RB: Bit,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_greater(self, _rhs: Frac!(- UInt<RU, RB>, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LN, LD, RN, RD> IsGreater<Frac!(RN, RD)> for Frac!(-LN, LD)
// where
//     LN: Unsigned,
//     LD: Unsigned + NonZero,
//     RN: Unsigned,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_greater(self, _rhs: Frac!(RN, RD)) -> Self::Output {
//         B0
//     }
// }

// is greater or equal

// impl<LF, RF> IsGreaterOrEqual<PFrac<RF>> for PFrac<LF>
// where
//     LF: UFraction + IsGreaterOrEqual<RF>,
//     RF: UFraction,
// {
//     type Output = GrEq<LF, RF>;

//     fn is_greater_or_equal(self, _rhs: PFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LF, RF> IsGreaterOrEqual<NFrac<RF>> for NFrac<LF>
// where
//     LF: UFraction + IsLessOrEqual<RF>,
//     RF: UFraction,
// {
//     type Output = LeEq<LF, RF>;

//     fn is_greater_or_equal(self, _rhs: NFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LD, RD> IsGreaterOrEqual<Frac!(UTerm, RD)> for Frac!(-UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_greater_or_equal(self, _rhs: Frac!(UTerm, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LD, RU, RB, RD> IsGreaterOrEqual<Frac!(UInt<RU, RB>, RD)> for Frac!(-UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RU: Unsigned,
//     RB: Bit,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_greater_or_equal(self, _rhs: Frac!(UInt<RU, RB>, RD)) -> Self::Output {
//         B0
//     }
// }

// impl<LU, LB, LD, RD> IsGreaterOrEqual<Frac!(UTerm, RD)> for Frac!(- UInt<LU, LB>, LD)
// where
//     LU: Unsigned,
//     LB: Bit,
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_greater_or_equal(self, _rhs: Frac!(UTerm, RD)) -> Self::Output {
//         B0
//     }
// }

// impl<LU, LB, LD, RU, RB, RD> IsGreaterOrEqual<Frac!(UInt<RU, RB>, RD)> for Frac!(- UInt<LU, LB>, LD)
// where
//     LU: Unsigned,
//     LB: Bit,
//     LD: Unsigned + NonZero,
//     RU: Unsigned,
//     RB: Bit,
//     RD: Unsigned + NonZero,
// {
//     type Output = B0;

//     fn is_greater_or_equal(self, _rhs: Frac!(UInt<RU, RB>, RD)) -> Self::Output {
//         B0
//     }
// }

// impl<LN, LD, RN, RD> IsGreaterOrEqual<Frac!(-RN, RD)> for Frac!(LN, LD)
// where
//     LN: Unsigned,
//     LD: Unsigned + NonZero,
//     RN: Unsigned,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_greater_or_equal(self, _rhs: Frac!(-RN, RD)) -> Self::Output {
//         B1
//     }
// }

// is equal

// impl<LF, RF> IsEqual<PFrac<RF>> for PFrac<LF>
// where
//     LF: UFraction + IsEqual<RF>,
//     RF: UFraction,
// {
//     type Output = Eq<LF, RF>;

//     fn is_equal(self, _rhs: PFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LF, RF> IsEqual<NFrac<RF>> for NFrac<LF>
// where
//     LF: UFraction + IsEqual<RF>,
//     RF: UFraction,
// {
//     type Output = Eq<LF, RF>;

//     fn is_equal(self, _rhs: NFrac<RF>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// impl<LD, RD> IsEqual<Frac!(UTerm, RD)> for Frac!(-UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_equal(self, _rhs: Frac!(UTerm, RD)) -> Self::Output {
//         B1
//     }
// }

// impl<LD, RD> IsEqual<Frac!(-UTerm, RD)> for Frac!(UTerm, LD)
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
// {
//     type Output = B1;

//     fn is_equal(self, _rhs: Frac!(-UTerm, RD)) -> Self::Output {
//         B1
//     }
// }
