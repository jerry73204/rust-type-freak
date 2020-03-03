use crate::numeric;
use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul},
};
use typenum::{NonZero, Prod, Quot, Sum, Unsigned};

// singed fraction trait

pub trait Fraction {
    fn new() -> Self;
}

// unsinged fraction trait

pub trait UFraction {
    fn new() -> Self;
}

// unsigned fraction

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

impl<N, D> NonZero for UFrac<N, D>
where
    N: Unsigned + NonZero,
    D: Unsigned + NonZero,
{
}

// TODO: sum of unsigned fractions

// impl<LN, LD, RN, RD> Add<UFrac<RN, RD>> for UFrac<LN, LD> {
//     type TmpD = Prod<LD, RD>;
//     type TmpN = Sum<Prod<LN, RD>, Prod<RN, LD>>;
//     type TmpGcd = numeric::op_aliases::Gcd<N, D>;
//     type OutN = Quot<TmpN, TmpGcd>;
//     type OutD = Quot<TmpD, TmpGcd>;
//     type Output = UFrac<OutN, OutD>;
// }

impl<LN, LD, RN, RD> Add<UFrac<RN, RD>> for UFrac<LN, LD>
where
    LD: Unsigned + NonZero + Mul<RD>,
    RD: Unsigned + NonZero,
    LN: Unsigned + Mul<RD>,
    RN: Unsigned + Mul<LD>,
    Prod<LN, RD>: Unsigned + Add<Prod<RN, LD>>,
    Sum<Prod<LN, RD>, Prod<RN, LD>>: Unsigned + numeric::ops::Gcd<Prod<LD, RD>>,
    Prod<LD, RD>:
        Unsigned + Div<numeric::op_aliases::Gcd<Sum<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>>,
    Sum<Prod<LN, RD>, Prod<RN, LD>>:
        Unsigned + Div<numeric::op_aliases::Gcd<Sum<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>>,
    Quot<
        Sum<Prod<LN, RD>, Prod<RN, LD>>,
        numeric::op_aliases::Gcd<Sum<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>,
    >: Unsigned,
    Quot<Prod<LD, RD>, numeric::op_aliases::Gcd<Sum<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>>:
        Unsigned + NonZero,
{
    type Output = UFrac<
        Quot<
            Sum<Prod<LN, RD>, Prod<RN, LD>>,
            numeric::op_aliases::Gcd<Sum<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>,
        >,
        Quot<Prod<LD, RD>, numeric::op_aliases::Gcd<Sum<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>>,
    >;

    fn add(self, _rhs: UFrac<RN, RD>) -> Self::Output {
        UFrac::new()
    }
}

// TODO: subtraction of unsigned fractions

// product of unsigned fractions

impl<LN, LD, RN, RD> Mul<UFrac<RN, RD>> for UFrac<LN, LD>
where
    LN: Unsigned + Mul<RN>,
    LD: Unsigned + Mul<RD> + NonZero,
    RN: Unsigned,
    RD: Unsigned + NonZero,
    Prod<LN, RN>: Unsigned
        + numeric::ops::Gcd<Prod<LD, RD>>
        + Div<numeric::op_aliases::Gcd<Prod<LN, RN>, Prod<LD, RD>>>,
    Prod<LD, RD>: Unsigned + Div<numeric::op_aliases::Gcd<Prod<LN, RN>, Prod<LD, RD>>>,
    Quot<Prod<LN, RN>, numeric::op_aliases::Gcd<Prod<LN, RN>, Prod<LD, RD>>>: Unsigned,
    Quot<Prod<LD, RD>, numeric::op_aliases::Gcd<Prod<LN, RN>, Prod<LD, RD>>>: Unsigned + NonZero,
{
    type Output = UFrac<
        Quot<Prod<LN, RN>, numeric::op_aliases::Gcd<Prod<LN, RN>, Prod<LD, RD>>>,
        Quot<Prod<LD, RD>, numeric::op_aliases::Gcd<Prod<LN, RN>, Prod<LD, RD>>>,
    >;

    fn mul(self, _rhs: UFrac<RN, RD>) -> Self::Output {
        UFrac::new()
    }
}

// division of unsigned fractions

impl<LN, LD, RN, RD> Div<UFrac<RN, RD>> for UFrac<LN, LD>
where
    LN: Unsigned + Mul<RD> + NonZero,
    LD: Unsigned + Mul<RN> + NonZero,
    RN: Unsigned,
    RD: Unsigned + NonZero,
    Prod<LN, RD>: Unsigned
        + Div<numeric::op_aliases::Gcd<Prod<LN, RD>, Prod<LD, RN>>>
        + numeric::ops::Gcd<Prod<LD, RN>>,
    Prod<LD, RN>: Unsigned + Div<numeric::op_aliases::Gcd<Prod<LN, RD>, Prod<LD, RN>>>,
    Quot<Prod<LN, RD>, numeric::op_aliases::Gcd<Prod<LN, RD>, Prod<LD, RN>>>: Unsigned,
    Quot<Prod<LD, RN>, numeric::op_aliases::Gcd<Prod<LN, RD>, Prod<LD, RN>>>: Unsigned + NonZero,
{
    type Output = UFrac<
        Quot<Prod<LN, RD>, numeric::op_aliases::Gcd<Prod<LN, RD>, Prod<LD, RN>>>,
        Quot<Prod<LD, RN>, numeric::op_aliases::Gcd<Prod<LN, RD>, Prod<LD, RN>>>,
    >;

    fn div(self, _rhs: UFrac<RN, RD>) -> Self::Output {
        UFrac::new()
    }
}

// positive fraction

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

impl<Frac> NonZero for PFrac<Frac> where Frac: UFraction + NonZero {}

// negative fraction

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

impl<Frac> NonZero for NFrac<Frac> where Frac: UFraction + NonZero {}

// products of positive/negative fractions

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

// TODO: sum of singed fractions

// TODO: subtraction of signed fractions

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::op_aliases::AssertSame;
    use typenum::{U1, U2, U27, U3, U35, U36, U4, U6, U8, U9};

    type Frac1 = UFrac<U3, U4>;
    type Frac2 = UFrac<U2, U9>;
    type Frac3 = Prod<Frac1, Frac2>;
    type Frac4 = Quot<Frac1, Frac2>;
    type Frac5 = Sum<Frac1, Frac2>;
    type Frac6 = Sum<Frac1, Frac1>;

    type Assert1 = AssertSame<Frac3, UFrac<U1, U6>, ()>;
    type Assert2 = AssertSame<Frac4, UFrac<U27, U8>, ()>;
    type Assert3 = AssertSame<Frac5, UFrac<U35, U36>, ()>;
    type Assert4 = AssertSame<Frac6, UFrac<U3, U2>, ()>;

    #[test]
    fn frac_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
    }
}
