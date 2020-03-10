use super::{
    marker::{Fraction, Irreducible, UFraction},
    signed::{NFrac, PFrac},
    unsigned::UFrac,
};
use crate::numeric;
use std::ops::{Add, Div, Mul, Sub};

use typenum::{Diff, NonZero, Prod, Quot, Sum, Unsigned};

pub mod ops {
    use super::*;

    // common denominator

    pub trait CommonDenominator<Rhs> {
        type Output;
    }

    impl<LN, LD, RN, RD> CommonDenominator<UFrac<RN, RD>> for UFrac<LN, LD>
    where
        LN: Unsigned,
        LD: Unsigned + NonZero + Mul<RD>,
        RN: Unsigned,
        RD: Unsigned + NonZero,
    {
        type Output = Prod<LD, RD>;
    }

    impl<LF, RF> CommonDenominator<PFrac<LF>> for PFrac<RF>
    where
        LF: UFraction + CommonDenominator<RF>,
        RF: UFraction,
    {
        type Output = op_aliases::CommonDenominator<LF, RF>;
    }

    impl<LF, RF> CommonDenominator<PFrac<LF>> for NFrac<RF>
    where
        LF: UFraction + CommonDenominator<RF>,
        RF: UFraction,
    {
        type Output = op_aliases::CommonDenominator<LF, RF>;
    }

    impl<LF, RF> CommonDenominator<NFrac<LF>> for PFrac<RF>
    where
        LF: UFraction + CommonDenominator<RF>,
        RF: UFraction,
    {
        type Output = op_aliases::CommonDenominator<LF, RF>;
    }

    impl<LF, RF> CommonDenominator<NFrac<LF>> for NFrac<RF>
    where
        LF: UFraction + CommonDenominator<RF>,
        RF: UFraction,
    {
        type Output = op_aliases::CommonDenominator<LF, RF>;
    }

    // reciprocal

    pub trait Reciprocal {
        type Output;
    }

    impl<N, D> Reciprocal for UFrac<N, D>
    where
        N: Unsigned + NonZero,
        D: Unsigned + NonZero,
    {
        type Output = UFrac<D, N>;
    }

    impl<F> Reciprocal for PFrac<F>
    where
        F: UFraction + Reciprocal,
        op_aliases::Reciprocal<F>: UFraction,
    {
        type Output = PFrac<op_aliases::Reciprocal<F>>;
    }

    impl<F> Reciprocal for NFrac<F>
    where
        F: UFraction + Reciprocal,
        op_aliases::Reciprocal<F>: UFraction,
    {
        type Output = NFrac<op_aliases::Reciprocal<F>>;
    }

    // reduce

    pub trait Reduce
    where
        Self::Output: Irreducible,
    {
        type Output;
    }

    impl<N, D> Reduce for UFrac<N, D>
    where
        N: Unsigned + numeric::ops::Gcd<D> + Div<numeric::op_aliases::Gcd<N, D>>,
        D: Unsigned + NonZero + Div<numeric::op_aliases::Gcd<N, D>>,
        Quot<N, numeric::op_aliases::Gcd<N, D>>: Unsigned,
        Quot<D, numeric::op_aliases::Gcd<N, D>>: Unsigned + NonZero,
        UFrac<Quot<N, numeric::op_aliases::Gcd<N, D>>, Quot<D, numeric::op_aliases::Gcd<N, D>>>:
            Irreducible,
    {
        type Output =
            UFrac<Quot<N, numeric::op_aliases::Gcd<N, D>>, Quot<D, numeric::op_aliases::Gcd<N, D>>>;
    }

    impl<F> Reduce for PFrac<F>
    where
        F: UFraction + Reduce,
        op_aliases::Reduce<F>: UFraction,
        PFrac<op_aliases::Reduce<F>>: Irreducible,
    {
        type Output = PFrac<op_aliases::Reduce<F>>;
    }

    impl<F> Reduce for NFrac<F>
    where
        F: UFraction + Reduce,
        op_aliases::Reduce<F>: UFraction,
        NFrac<op_aliases::Reduce<F>>: Irreducible,
    {
        type Output = NFrac<op_aliases::Reduce<F>>;
    }

    // add without reduction

    pub trait NaiveAdd<Rhs> {
        type Output;
    }

    impl<LN, LD, RN, RD> NaiveAdd<UFrac<RN, RD>> for UFrac<LN, LD>
    where
        LD: Unsigned + NonZero + Mul<RD>,
        RD: Unsigned + NonZero,
        LN: Unsigned + Mul<RD>,
        RN: Unsigned + Mul<LD>,
        Prod<LN, RD>: Add<Prod<RN, LD>>,
        Prod<LD, RD>: NonZero + Unsigned,
        Sum<Prod<LN, RD>, Prod<RN, LD>>: Unsigned,
    {
        type Output = UFrac<Sum<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>;
    }

    impl<LF, RF> NaiveAdd<PFrac<LF>> for PFrac<RF>
    where
        LF: UFraction + NaiveAdd<RF>,
        RF: UFraction,
        op_aliases::NaiveAdd<LF, RF>: UFraction,
    {
        type Output = PFrac<op_aliases::NaiveAdd<LF, RF>>;
    }

    impl<LF, RF> NaiveAdd<NFrac<LF>> for NFrac<RF>
    where
        LF: UFraction + NaiveAdd<RF>,
        RF: UFraction,
        op_aliases::NaiveAdd<LF, RF>: UFraction,
    {
        type Output = NFrac<op_aliases::NaiveAdd<LF, RF>>;
    }

    // subtraction without reduction

    pub trait NaiveSub<Rhs> {
        type Output;
    }

    impl<LN, LD, RN, RD> NaiveSub<UFrac<RN, RD>> for UFrac<LN, LD>
    where
        LD: Unsigned + NonZero + Mul<RD>,
        RD: Unsigned + NonZero,
        LN: Unsigned + Mul<RD>,
        RN: Unsigned + Mul<LD>,
        Prod<LN, RD>: Sub<Prod<RN, LD>>,
        Diff<Prod<LN, RD>, Prod<RN, LD>>: Unsigned,
        Prod<LD, RD>: Unsigned + NonZero,
    {
        type Output = UFrac<Diff<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>;
    }

    // multiply without reduction

    pub trait NaiveMul<Rhs> {
        type Output;
    }

    impl<LN, LD, RN, RD> NaiveMul<UFrac<RN, RD>> for UFrac<LN, LD>
    where
        LD: Unsigned + NonZero + Mul<RD>,
        RD: Unsigned + NonZero,
        LN: Unsigned + Mul<RN>,
        RN: Unsigned,
        Prod<LN, RN>: Unsigned,
        Prod<LD, RD>: Unsigned + NonZero,
    {
        type Output = UFrac<Prod<LN, RN>, Prod<LD, RD>>;
    }

    impl<LF, RF> NaiveMul<PFrac<LF>> for PFrac<RF>
    where
        LF: UFraction + NaiveMul<RF>,
        RF: UFraction,
        op_aliases::NaiveMul<LF, RF>: UFraction,
    {
        type Output = PFrac<op_aliases::NaiveMul<LF, RF>>;
    }

    impl<LF, RF> NaiveMul<PFrac<LF>> for NFrac<RF>
    where
        LF: UFraction + NaiveMul<RF>,
        RF: UFraction,
        op_aliases::NaiveMul<LF, RF>: UFraction,
    {
        type Output = NFrac<op_aliases::NaiveMul<LF, RF>>;
    }

    impl<LF, RF> NaiveMul<NFrac<LF>> for PFrac<RF>
    where
        LF: UFraction + NaiveMul<RF>,
        RF: UFraction,
        op_aliases::NaiveMul<LF, RF>: UFraction,
    {
        type Output = NFrac<op_aliases::NaiveMul<LF, RF>>;
    }

    impl<LF, RF> NaiveMul<NFrac<LF>> for NFrac<RF>
    where
        LF: UFraction + NaiveMul<RF>,
        RF: UFraction,
        op_aliases::NaiveMul<LF, RF>: UFraction,
    {
        type Output = PFrac<op_aliases::NaiveMul<LF, RF>>;
    }

    // division without reduction

    pub trait NaiveDiv<Rhs> {
        type Output;
    }

    impl<LN, LD, RN, RD> NaiveDiv<UFrac<RN, RD>> for UFrac<LN, LD>
    where
        LD: Unsigned + NonZero,
        RD: Unsigned + NonZero,
        LN: Unsigned,
        RN: Unsigned,
        UFrac<LN, LD>: NaiveMul<op_aliases::Reciprocal<UFrac<RN, RD>>>,
        UFrac<RN, RD>: Reciprocal,
    {
        type Output = op_aliases::NaiveMul<UFrac<LN, LD>, op_aliases::Reciprocal<UFrac<RN, RD>>>;
    }

    // satuating subtraction without reduction

    pub trait NaiveSaturatingSub<Rhs>
    where
        Self: UFraction,
        Self::Output: UFraction,
    {
        type Output;
    }

    impl<LN, LD, RN, RD> NaiveSaturatingSub<UFrac<RN, RD>> for UFrac<LN, LD>
    where
        LD: Unsigned + NonZero + Mul<RD>,
        RD: Unsigned + NonZero,
        LN: Unsigned + Mul<RD>,
        RN: Unsigned + Mul<LD>,
        Prod<LN, RD>: numeric::ops::SaturatingSub<Prod<RN, LD>>,
        numeric::op_aliases::SaturatingSub<Prod<LN, RD>, Prod<RN, LD>>: Unsigned,
        Prod<LD, RD>: Unsigned + NonZero,
    {
        type Output =
            UFrac<numeric::op_aliases::SaturatingSub<Prod<LN, RD>, Prod<RN, LD>>, Prod<LD, RD>>;
    }

    // satuating subtraction with reduction

    pub trait SaturatingSub<Rhs>
    where
        Self: UFraction,
        Self::Output: UFraction,
    {
        type Output;
    }

    impl<LN, LD, RN, RD> SaturatingSub<UFrac<RN, RD>> for UFrac<LN, LD>
    where
        LD: Unsigned + NonZero,
        RD: Unsigned + NonZero,
        LN: Unsigned,
        RN: Unsigned,
        UFrac<LN, LD>: NaiveSaturatingSub<UFrac<RN, RD>>,
        op_aliases::NaiveSaturatingSub<UFrac<LN, LD>, UFrac<RN, RD>>: Reduce,
        op_aliases::Reduce<op_aliases::NaiveSaturatingSub<UFrac<LN, LD>, UFrac<RN, RD>>>: UFraction,
    {
        type Output =
            op_aliases::Reduce<op_aliases::NaiveSaturatingSub<UFrac<LN, LD>, UFrac<RN, RD>>>;
    }

}

pub mod op_aliases {
    use super::*;

    pub type CommonDenominator<Lhs, Rhs> = <Lhs as ops::CommonDenominator<Rhs>>::Output;
    pub type Reduce<F> = <F as ops::Reduce>::Output;
    pub type Reciprocal<F> = <F as ops::Reciprocal>::Output;
    pub type NaiveAdd<Lhs, Rhs> = <Lhs as ops::NaiveAdd<Rhs>>::Output;
    pub type NaiveSub<Lhs, Rhs> = <Lhs as ops::NaiveSub<Rhs>>::Output;
    pub type NaiveMul<Lhs, Rhs> = <Lhs as ops::NaiveMul<Rhs>>::Output;
    pub type NaiveDiv<Lhs, Rhs> = <Lhs as ops::NaiveDiv<Rhs>>::Output;
    pub type NaiveSaturatingSub<Lhs, Rhs> = <Lhs as ops::NaiveSaturatingSub<Rhs>>::Output;
    pub type SaturatingSub<Lhs, Rhs> = <Lhs as ops::SaturatingSub<Rhs>>::Output;

}

#[cfg(test)]
mod tests {
    use super::{op_aliases::*, *};
    use crate::control::op_aliases::AssertSame;
    use typenum::{U0, U1, U12, U16, U2, U24, U3, U36, U4, U6, U9};

    type Frac1 = UFrac<U3, U4>;
    type Frac2 = UFrac<U2, U9>;
    type Frac11 = UFrac<U9, U4>;
    type Frac3 = NaiveMul<Frac1, Frac2>;
    type Frac4 = NaiveDiv<Frac1, Frac11>;
    type Frac5 = NaiveAdd<Frac1, Frac1>;
    type Frac7 = Reduce<UFrac<U2, U4>>;
    type Frac8 = Reduce<UFrac<U0, U4>>;
    type Frac9 = Reduce<PFrac<UFrac<U3, U9>>>;
    type Frac10 = Reciprocal<UFrac<U2, U3>>;

    type Assert1 = AssertSame<Frac3, UFrac<U6, U36>, ()>;
    type Assert2 = AssertSame<Frac4, UFrac<U12, U36>, ()>;
    type Assert3 = AssertSame<Frac5, UFrac<U24, U16>, ()>;
    type Assert5 = AssertSame<Frac7, UFrac<U1, U2>, ()>;
    type Assert6 = AssertSame<Frac8, UFrac<U0, U1>, ()>;
    type Assert7 = AssertSame<Frac9, PFrac<UFrac<U1, U3>>, ()>;
    type Assert8 = AssertSame<Frac10, UFrac<U3, U2>, ()>;

    #[test]
    fn frac_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
    }
}
