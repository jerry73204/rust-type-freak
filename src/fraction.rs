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
