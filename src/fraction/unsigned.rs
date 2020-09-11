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

// is less

// impl<LN, LD, RN, RD> IsLess<UFrac<RN, RD>> for UFrac<LN, LD>
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
//     LN: Unsigned + Mul<RD>,
//     RN: Unsigned + Mul<LD>,
//     Prod<LN, RD>: IsLess<Prod<RN, LD>>,
// {
//     type Output = Le<Prod<LN, RD>, Prod<RN, LD>>;

//     fn is_less(self, _rhs: UFrac<RN, RD>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// is less or equal

// impl<LN, LD, RN, RD> IsLessOrEqual<UFrac<RN, RD>> for UFrac<LN, LD>
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
//     LN: Unsigned + Mul<RD>,
//     RN: Unsigned + Mul<LD>,
//     Prod<LN, RD>: IsLessOrEqual<Prod<RN, LD>>,
// {
//     type Output = LeEq<Prod<LN, RD>, Prod<RN, LD>>;

//     fn is_less_or_equal(self, _rhs: UFrac<RN, RD>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// is greater

// impl<LN, LD, RN, RD> IsGreater<UFrac<RN, RD>> for UFrac<LN, LD>
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
//     LN: Unsigned + Mul<RD>,
//     RN: Unsigned + Mul<LD>,
//     Prod<LN, RD>: IsGreater<Prod<RN, LD>>,
// {
//     type Output = Gr<Prod<LN, RD>, Prod<RN, LD>>;

//     fn is_greater(self, _rhs: UFrac<RN, RD>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// is greater or equal

// impl<LN, LD, RN, RD> IsGreaterOrEqual<UFrac<RN, RD>> for UFrac<LN, LD>
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
//     LN: Unsigned + Mul<RD>,
//     RN: Unsigned + Mul<LD>,
//     Prod<LN, RD>: IsGreaterOrEqual<Prod<RN, LD>>,
// {
//     type Output = GrEq<Prod<LN, RD>, Prod<RN, LD>>;

//     fn is_greater_or_equal(self, _rhs: UFrac<RN, RD>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// is equal

// impl<LN, LD, RN, RD> IsEqual<UFrac<RN, RD>> for UFrac<LN, LD>
// where
//     LD: Unsigned + NonZero,
//     RD: Unsigned + NonZero,
//     LN: Unsigned + Mul<RD>,
//     RN: Unsigned + Mul<LD>,
//     Prod<LN, RD>: IsEqual<Prod<RN, LD>>,
// {
//     type Output = Eq<Prod<LN, RD>, Prod<RN, LD>>;

//     fn is_equal(self, _rhs: UFrac<RN, RD>) -> Self::Output {
//         // blocked by https://github.com/paholg/typenum/pull/141
//         todo!();
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::{op_aliases::*, *};
//     use crate::control::op_aliases::AssertSame;
//     use typenum::{U0, U1, U2, U27, U3, U35, U36, U4, U6, U8, U9};

//     type Frac1 = UFrac<U3, U4>;
//     type Frac2 = UFrac<U2, U9>;
//     type Frac3 = Prod<Frac1, Frac2>;
//     type Frac4 = Quot<Frac1, Frac2>;
//     type Frac5 = Sum<Frac1, Frac2>;
//     type Frac6 = Sum<Frac1, Frac1>;

//     type Assert1 = AssertSame<Frac3, UFrac<U1, U6>, ()>;
//     type Assert2 = AssertSame<Frac4, UFrac<U27, U8>, ()>;
//     type Assert3 = AssertSame<Frac5, UFrac<U35, U36>, ()>;
//     type Assert4 = AssertSame<Frac6, UFrac<U3, U2>, ()>;

//     #[test]
//     fn frac_test() {
//         let _: Assert1 = ();
//         let _: Assert2 = ();
//         let _: Assert3 = ();
//         let _: Assert4 = ();
//     }
// }
