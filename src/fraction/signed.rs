use super::{
    marker::{Fraction, Irreducible, UFraction},
    op_aliases, ops,
};
use crate::{control, FracT};
use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use typenum::{
    Bit, Diff, Eq, Gr, GrEq, IsEqual, IsGreater, IsGreaterOrEqual, IsLess, IsLessOrEqual, Le, LeEq,
    NonZero, Prod, Quot, Sum, UInt, UTerm, Unsigned, B0, B1,
};

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

// sum of singed fractions

impl<LF, RF> Add<PFrac<RF>> for PFrac<LF>
where
    LF: UFraction + Add<RF>,
    RF: UFraction,
    Sum<LF, RF>: UFraction,
{
    type Output = PFrac<Sum<LF, RF>>;

    fn add(self, _rhs: PFrac<RF>) -> Self::Output {
        PFrac::new()
    }
}

impl<LF, RF> Add<NFrac<RF>> for NFrac<LF>
where
    LF: UFraction + Add<RF>,
    RF: UFraction,
    Sum<LF, RF>: UFraction,
{
    type Output = NFrac<Sum<LF, RF>>;

    fn add(self, _rhs: NFrac<RF>) -> Self::Output {
        NFrac::new()
    }
}

impl<LF, RF> Add<NFrac<RF>> for PFrac<LF>
where
    LF: UFraction,
    RF: UFraction,
    PFrac<LF>: Sub<PFrac<RF>>,
    Diff<PFrac<LF>, PFrac<RF>>: Fraction,
{
    type Output = Diff<PFrac<LF>, PFrac<RF>>;

    fn add(self, _rhs: NFrac<RF>) -> Self::Output {
        Self::Output::new()
    }
}

impl<LF, RF> Add<PFrac<RF>> for NFrac<LF>
where
    LF: UFraction,
    RF: UFraction,
    PFrac<RF>: Sub<PFrac<LF>>,
    Diff<PFrac<RF>, PFrac<LF>>: Fraction,
{
    type Output = Diff<PFrac<RF>, PFrac<LF>>;

    fn add(self, _rhs: PFrac<RF>) -> Self::Output {
        Self::Output::new()
    }
}

// subtraction of signed fractions

impl<LF, RF> Sub<PFrac<RF>> for PFrac<LF>
where
    (): control::ops::IfElseGreaterOrEqual<
        LF,
        RF,
        PFrac<op_aliases::SaturatingSub<LF, RF>>,
        NFrac<op_aliases::SaturatingSub<RF, LF>>,
    >,
    LF: UFraction + ops::SaturatingSub<RF>,
    RF: UFraction + ops::SaturatingSub<LF>,
    control::op_aliases::IfElseGreaterOrEqual<
        LF,
        RF,
        PFrac<op_aliases::SaturatingSub<LF, RF>>,
        NFrac<op_aliases::SaturatingSub<RF, LF>>,
    >: Fraction,
{
    type Output = control::op_aliases::IfElseGreaterOrEqual<
        LF,
        RF,
        PFrac<op_aliases::SaturatingSub<LF, RF>>,
        NFrac<op_aliases::SaturatingSub<RF, LF>>,
    >;

    fn sub(self, _rhs: PFrac<RF>) -> Self::Output {
        Self::Output::new()
    }
}

impl<LF, RF> Sub<NFrac<RF>> for NFrac<LF>
where
    (): control::ops::IfElseGreaterOrEqual<
        LF,
        RF,
        NFrac<op_aliases::SaturatingSub<LF, RF>>,
        PFrac<op_aliases::SaturatingSub<RF, LF>>,
    >,
    LF: UFraction + ops::SaturatingSub<RF>,
    RF: UFraction + ops::SaturatingSub<LF>,
    control::op_aliases::IfElseGreaterOrEqual<
        LF,
        RF,
        NFrac<op_aliases::SaturatingSub<LF, RF>>,
        PFrac<op_aliases::SaturatingSub<RF, LF>>,
    >: Fraction,
{
    type Output = control::op_aliases::IfElseGreaterOrEqual<
        LF,
        RF,
        NFrac<op_aliases::SaturatingSub<LF, RF>>,
        PFrac<op_aliases::SaturatingSub<RF, LF>>,
    >;

    fn sub(self, _rhs: NFrac<RF>) -> Self::Output {
        Self::Output::new()
    }
}

impl<LF, RF> Sub<NFrac<RF>> for PFrac<LF>
where
    LF: UFraction,
    RF: UFraction,
    PFrac<LF>: Add<PFrac<RF>>,
    Sum<PFrac<LF>, PFrac<RF>>: Fraction,
{
    type Output = Sum<PFrac<LF>, PFrac<RF>>;

    fn sub(self, _rhs: NFrac<RF>) -> Self::Output {
        Self::Output::new()
    }
}

impl<LF, RF> Sub<PFrac<RF>> for NFrac<LF>
where
    LF: UFraction,
    RF: UFraction,
    NFrac<RF>: Add<NFrac<LF>>,
    Sum<NFrac<RF>, NFrac<LF>>: Fraction,
{
    type Output = Sum<NFrac<RF>, NFrac<LF>>;

    fn sub(self, _rhs: PFrac<RF>) -> Self::Output {
        Self::Output::new()
    }
}

// products

impl<LF, RF> Mul<PFrac<RF>> for PFrac<LF>
where
    LF: UFraction + Mul<RF>,
    RF: UFraction,
    Prod<LF, RF>: UFraction,
{
    type Output = PFrac<Prod<LF, RF>>;

    fn mul(self, _rhs: PFrac<RF>) -> Self::Output {
        PFrac::new()
    }
}

impl<LF, RF> Mul<NFrac<RF>> for PFrac<LF>
where
    LF: UFraction + Mul<RF>,
    RF: UFraction,
    Prod<LF, RF>: UFraction,
{
    type Output = NFrac<Prod<LF, RF>>;

    fn mul(self, _rhs: NFrac<RF>) -> Self::Output {
        NFrac::new()
    }
}

impl<LF, RF> Mul<PFrac<RF>> for NFrac<LF>
where
    LF: UFraction + Mul<RF>,
    RF: UFraction,
    Prod<LF, RF>: UFraction,
{
    type Output = NFrac<Prod<LF, RF>>;

    fn mul(self, _rhs: PFrac<RF>) -> Self::Output {
        NFrac::new()
    }
}

impl<LF, RF> Mul<NFrac<RF>> for NFrac<LF>
where
    LF: UFraction + Mul<RF>,
    RF: UFraction,
    Prod<LF, RF>: UFraction,
{
    type Output = PFrac<Prod<LF, RF>>;

    fn mul(self, _rhs: NFrac<RF>) -> Self::Output {
        PFrac::new()
    }
}

// division of positive/negative fractions

impl<LF, RF> Div<PFrac<RF>> for PFrac<LF>
where
    LF: UFraction + Div<RF>,
    RF: UFraction,
    PFrac<RF>: NonZero,
    Quot<LF, RF>: UFraction,
{
    type Output = PFrac<Quot<LF, RF>>;

    fn div(self, _rhs: PFrac<RF>) -> Self::Output {
        PFrac::new()
    }
}

impl<LF, RF> Div<NFrac<RF>> for PFrac<LF>
where
    LF: UFraction + Div<RF>,
    RF: UFraction,
    NFrac<RF>: NonZero,
    Quot<LF, RF>: UFraction,
{
    type Output = NFrac<Quot<LF, RF>>;

    fn div(self, _rhs: NFrac<RF>) -> Self::Output {
        NFrac::new()
    }
}

impl<LF, RF> Div<PFrac<RF>> for NFrac<LF>
where
    LF: UFraction + Div<RF>,
    RF: UFraction,
    PFrac<RF>: NonZero,
    Quot<LF, RF>: UFraction,
{
    type Output = NFrac<Quot<LF, RF>>;

    fn div(self, _rhs: PFrac<RF>) -> Self::Output {
        NFrac::new()
    }
}

impl<LF, RF> Div<NFrac<RF>> for NFrac<LF>
where
    LF: UFraction + Div<RF>,
    RF: UFraction,
    NFrac<RF>: NonZero,
    Quot<LF, RF>: UFraction,
{
    type Output = PFrac<Quot<LF, RF>>;

    fn div(self, _rhs: NFrac<RF>) -> Self::Output {
        PFrac::new()
    }
}

// is less

impl<LF, RF> IsLess<PFrac<RF>> for PFrac<LF>
where
    LF: UFraction + IsLess<RF>,
    RF: UFraction,
{
    type Output = Le<LF, RF>;

    fn is_less(self, _rhs: PFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LF, RF> IsLess<NFrac<RF>> for NFrac<LF>
where
    LF: UFraction + IsGreater<RF>,
    RF: UFraction,
{
    type Output = Gr<LF, RF>;

    fn is_less(self, _rhs: NFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LD, RD> IsLess<FracT!(UTerm, RD)> for FracT!(-UTerm, LD)
where
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_less(self, _rhs: FracT!(UTerm, RD)) -> Self::Output {
        B0
    }
}

impl<LD, RU, RB, RD> IsLess<FracT!(UInt<RU, RB>, RD)> for FracT!(-UTerm, LD)
where
    LD: Unsigned + NonZero,
    RU: Unsigned,
    RB: Bit,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_less(self, _rhs: FracT!(UInt<RU, RB>, RD)) -> Self::Output {
        B1
    }
}

impl<LU, LB, LD, RD> IsLess<FracT!(UTerm, RD)> for FracT!(- UInt<LU, LB>, LD)
where
    LU: Unsigned,
    LB: Bit,
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_less(self, _rhs: FracT!(UTerm, RD)) -> Self::Output {
        B1
    }
}

impl<LU, LB, LD, RU, RB, RD> IsLess<FracT!(UInt<RU, RB>, RD)> for FracT!(- UInt<LU, LB>, LD)
where
    LU: Unsigned,
    LB: Bit,
    LD: Unsigned + NonZero,
    RU: Unsigned,
    RB: Bit,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_less(self, _rhs: FracT!(UInt<RU, RB>, RD)) -> Self::Output {
        B1
    }
}

impl<LN, LD, RN, RD> IsLess<FracT!(-RN, RD)> for FracT!(LN, LD)
where
    LN: Unsigned,
    LD: Unsigned + NonZero,
    RN: Unsigned,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_less(self, _rhs: FracT!(-RN, RD)) -> Self::Output {
        B0
    }
}

// is less or equal

impl<LF, RF> IsLessOrEqual<PFrac<RF>> for PFrac<LF>
where
    LF: UFraction + IsLessOrEqual<RF>,
    RF: UFraction,
{
    type Output = LeEq<LF, RF>;

    fn is_less_or_equal(self, _rhs: PFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LF, RF> IsLessOrEqual<NFrac<RF>> for NFrac<LF>
where
    LF: UFraction + IsGreaterOrEqual<RF>,
    RF: UFraction,
{
    type Output = GrEq<LF, RF>;

    fn is_less_or_equal(self, _rhs: NFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LD, RD> IsLessOrEqual<FracT!(-UTerm, RD)> for FracT!(UTerm, LD)
where
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_less_or_equal(self, _rhs: FracT!(-UTerm, RD)) -> Self::Output {
        B1
    }
}

impl<LD, RU, RB, RD> IsLessOrEqual<FracT!(- UInt<RU, RB>, RD)> for FracT!(UTerm, LD)
where
    LD: Unsigned + NonZero,
    RU: Unsigned,
    RB: Bit,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_less_or_equal(self, _rhs: FracT!(- UInt<RU, RB>, RD)) -> Self::Output {
        B0
    }
}

impl<LU, LB, LD, RD> IsLessOrEqual<FracT!(-UTerm, RD)> for FracT!(UInt<LU, LB>, LD)
where
    LU: Unsigned,
    LB: Bit,
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_less_or_equal(self, _rhs: FracT!(-UTerm, RD)) -> Self::Output {
        B0
    }
}

impl<LU, LB, LD, RU, RB, RD> IsLessOrEqual<FracT!(- UInt<RU, RB>, RD)> for FracT!(UInt<LU, LB>, LD)
where
    LU: Unsigned,
    LB: Bit,
    LD: Unsigned + NonZero,
    RU: Unsigned,
    RB: Bit,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_less_or_equal(self, _rhs: FracT!(- UInt<RU, RB>, RD)) -> Self::Output {
        B0
    }
}

impl<LN, LD, RN, RD> IsLessOrEqual<FracT!(RN, RD)> for FracT!(-LN, LD)
where
    LN: Unsigned,
    LD: Unsigned + NonZero,
    RN: Unsigned,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_less_or_equal(self, _rhs: FracT!(RN, RD)) -> Self::Output {
        B1
    }
}

// is greater

impl<LF, RF> IsGreater<PFrac<RF>> for PFrac<LF>
where
    LF: UFraction + IsGreater<RF>,
    RF: UFraction,
{
    type Output = Gr<LF, RF>;

    fn is_greater(self, _rhs: PFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LF, RF> IsGreater<NFrac<RF>> for NFrac<LF>
where
    LF: UFraction + IsLess<RF>,
    RF: UFraction,
{
    type Output = Le<LF, RF>;

    fn is_greater(self, _rhs: NFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LD, RD> IsGreater<FracT!(-UTerm, RD)> for FracT!(UTerm, LD)
where
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_greater(self, _rhs: FracT!(-UTerm, RD)) -> Self::Output {
        B0
    }
}

impl<LD, RU, RB, RD> IsGreater<FracT!(- UInt<RU, RB>, RD)> for FracT!(UTerm, LD)
where
    LD: Unsigned + NonZero,
    RU: Unsigned,
    RB: Bit,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_greater(self, _rhs: FracT!(- UInt<RU, RB>, RD)) -> Self::Output {
        B1
    }
}

impl<LU, LB, LD, RD> IsGreater<FracT!(-UTerm, RD)> for FracT!(UInt<LU, LB>, LD)
where
    LU: Unsigned,
    LB: Bit,
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_greater(self, _rhs: FracT!(-UTerm, RD)) -> Self::Output {
        B1
    }
}

impl<LU, LB, LD, RU, RB, RD> IsGreater<FracT!(- UInt<RU, RB>, RD)> for FracT!(UInt<LU, LB>, LD)
where
    LU: Unsigned,
    LB: Bit,
    LD: Unsigned + NonZero,
    RU: Unsigned,
    RB: Bit,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_greater(self, _rhs: FracT!(- UInt<RU, RB>, RD)) -> Self::Output {
        B1
    }
}

impl<LN, LD, RN, RD> IsGreater<FracT!(RN, RD)> for FracT!(-LN, LD)
where
    LN: Unsigned,
    LD: Unsigned + NonZero,
    RN: Unsigned,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_greater(self, _rhs: FracT!(RN, RD)) -> Self::Output {
        B0
    }
}

// is greater or equal

impl<LF, RF> IsGreaterOrEqual<PFrac<RF>> for PFrac<LF>
where
    LF: UFraction + IsGreaterOrEqual<RF>,
    RF: UFraction,
{
    type Output = GrEq<LF, RF>;

    fn is_greater_or_equal(self, _rhs: PFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LF, RF> IsGreaterOrEqual<NFrac<RF>> for NFrac<LF>
where
    LF: UFraction + IsLessOrEqual<RF>,
    RF: UFraction,
{
    type Output = LeEq<LF, RF>;

    fn is_greater_or_equal(self, _rhs: NFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LD, RD> IsGreaterOrEqual<FracT!(UTerm, RD)> for FracT!(-UTerm, LD)
where
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_greater_or_equal(self, _rhs: FracT!(UTerm, RD)) -> Self::Output {
        B1
    }
}

impl<LD, RU, RB, RD> IsGreaterOrEqual<FracT!(UInt<RU, RB>, RD)> for FracT!(-UTerm, LD)
where
    LD: Unsigned + NonZero,
    RU: Unsigned,
    RB: Bit,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_greater_or_equal(self, _rhs: FracT!(UInt<RU, RB>, RD)) -> Self::Output {
        B0
    }
}

impl<LU, LB, LD, RD> IsGreaterOrEqual<FracT!(UTerm, RD)> for FracT!(- UInt<LU, LB>, LD)
where
    LU: Unsigned,
    LB: Bit,
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_greater_or_equal(self, _rhs: FracT!(UTerm, RD)) -> Self::Output {
        B0
    }
}

impl<LU, LB, LD, RU, RB, RD> IsGreaterOrEqual<FracT!(UInt<RU, RB>, RD)> for FracT!(- UInt<LU, LB>, LD)
where
    LU: Unsigned,
    LB: Bit,
    LD: Unsigned + NonZero,
    RU: Unsigned,
    RB: Bit,
    RD: Unsigned + NonZero,
{
    type Output = B0;

    fn is_greater_or_equal(self, _rhs: FracT!(UInt<RU, RB>, RD)) -> Self::Output {
        B0
    }
}

impl<LN, LD, RN, RD> IsGreaterOrEqual<FracT!(-RN, RD)> for FracT!(LN, LD)
where
    LN: Unsigned,
    LD: Unsigned + NonZero,
    RN: Unsigned,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_greater_or_equal(self, _rhs: FracT!(-RN, RD)) -> Self::Output {
        B1
    }
}

// is equal

impl<LF, RF> IsEqual<PFrac<RF>> for PFrac<LF>
where
    LF: UFraction + IsEqual<RF>,
    RF: UFraction,
{
    type Output = Eq<LF, RF>;

    fn is_equal(self, _rhs: PFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LF, RF> IsEqual<NFrac<RF>> for NFrac<LF>
where
    LF: UFraction + IsEqual<RF>,
    RF: UFraction,
{
    type Output = Eq<LF, RF>;

    fn is_equal(self, _rhs: NFrac<RF>) -> Self::Output {
        // blocked by https://github.com/paholg/typenum/pull/141
        todo!();
    }
}

impl<LD, RD> IsEqual<FracT!(UTerm, RD)> for FracT!(-UTerm, LD)
where
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_equal(self, _rhs: FracT!(UTerm, RD)) -> Self::Output {
        B1
    }
}

impl<LD, RD> IsEqual<FracT!(-UTerm, RD)> for FracT!(UTerm, LD)
where
    LD: Unsigned + NonZero,
    RD: Unsigned + NonZero,
{
    type Output = B1;

    fn is_equal(self, _rhs: FracT!(-UTerm, RD)) -> Self::Output {
        B1
    }
}
