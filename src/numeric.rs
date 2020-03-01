//! Numeric type operators and functors.

use crate::control;
use std::ops::{Add, Mul, Sub};
use typenum::{Add1, Bit, Prod, Sub1, UInt, UTerm, Unsigned, B0, B1, U1, U2};

pub mod ops {
    use super::*;

    // saturating subtraction

    pub trait SaturatingSub<Rhs>
    where
        Self: Unsigned,
        Self::Output: Unsigned,
    {
        type Output;
    }

    impl SaturatingSub<UTerm> for UTerm {
        type Output = UTerm;
    }

    impl<U, B> SaturatingSub<UInt<U, B>> for UTerm
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = UTerm;
    }

    impl<U, B> SaturatingSub<UTerm> for UInt<U, B>
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = UInt<U, B>;
    }

    impl<UL, UR> SaturatingSub<UInt<UR, B1>> for UInt<UL, B1>
    where
        UL: Unsigned + SaturatingSub<UR>,
        UR: Unsigned,
        (): control::ops::IfElseGreater<UL, UR, UInt<op_aliases::SaturatingSub<UL, UR>, B0>, UTerm>,
        control::op_aliases::IfElseGreater<
            UL,
            UR,
            UInt<op_aliases::SaturatingSub<UL, UR>, B0>,
            UTerm,
        >: Unsigned,
    {
        type Output = control::op_aliases::IfElseGreater<
            UL,
            UR,
            UInt<op_aliases::SaturatingSub<UL, UR>, B0>,
            UTerm,
        >;
    }

    impl<UL, UR> SaturatingSub<UInt<UR, B0>> for UInt<UL, B0>
    where
        UL: Unsigned + SaturatingSub<UR>,
        UR: Unsigned,
        (): control::ops::IfElseGreater<UL, UR, UInt<op_aliases::SaturatingSub<UL, UR>, B0>, UTerm>,
        control::op_aliases::IfElseGreater<
            UL,
            UR,
            UInt<op_aliases::SaturatingSub<UL, UR>, B0>,
            UTerm,
        >: Unsigned,
    {
        type Output = control::op_aliases::IfElseGreater<
            UL,
            UR,
            UInt<op_aliases::SaturatingSub<UL, UR>, B0>,
            UTerm,
        >;
    }

    impl<UL, UR> SaturatingSub<UInt<UR, B0>> for UInt<UL, B1>
    where
        UL: Unsigned + SaturatingSub<UR>,
        UR: Unsigned,
        (): control::ops::IfElseGreaterOrEqual<
            UL,
            UR,
            UInt<op_aliases::SaturatingSub<UL, UR>, B1>,
            UTerm,
        >,
        control::op_aliases::IfElseGreaterOrEqual<
            UL,
            UR,
            UInt<op_aliases::SaturatingSub<UL, UR>, B1>,
            UTerm,
        >: Unsigned,
    {
        type Output = control::op_aliases::IfElseGreaterOrEqual<
            UL,
            UR,
            UInt<op_aliases::SaturatingSub<UL, UR>, B1>,
            UTerm,
        >;
    }

    impl<UL, UR> SaturatingSub<UInt<UR, B1>> for UInt<UL, B0>
    where
        UL: Unsigned + Sub<B1>,
        UR: Unsigned,
        UInt<Sub1<UL>, B1>: SaturatingSub<UInt<UR, B0>>,
    {
        type Output = op_aliases::SaturatingSub<UInt<Sub1<UL>, B1>, UInt<UR, B0>>;
    }

    // greatest common divisor

    pub trait Gcd<Rhs>
    where
        Self: Unsigned,
        Self::Output: Unsigned,
        Rhs: Unsigned,
    {
        type Output;
    }

    impl<U, B> Gcd<UInt<U, B>> for UTerm
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = UInt<U, B>;
    }

    impl<U, B> Gcd<UTerm> for UInt<U, B>
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = UInt<U, B>;
    }

    impl<UL, BL, UR, BR> Gcd<UInt<UR, BR>> for UInt<UL, BL>
    where
        UL: Unsigned,
        BL: Bit,
        UR: Unsigned,
        BR: Bit,
        UInt<UL, BL>: GcdWithFactor<UInt<UR, BR>, U1>,
    {
        type Output = op_aliases::GcdWithFactor<UInt<UL, BL>, UInt<UR, BR>, U1>;
    }

    // greatest common divisor with factor

    pub trait GcdWithFactor<Rhs, Factor>
    where
        Self: Unsigned,
        Self::Output: Unsigned,
        Rhs: Unsigned,
        Factor: Unsigned,
    {
        type Output;
    }

    impl<Factor> GcdWithFactor<UTerm, Factor> for UTerm
    where
        Factor: Unsigned,
    {
        type Output = Factor;
    }

    impl<Factor, U, B> GcdWithFactor<UInt<U, B>, Factor> for UTerm
    where
        U: Unsigned,
        B: Bit,
        Factor: Unsigned + Mul<UInt<U, B>>,
        Prod<Factor, UInt<U, B>>: Unsigned,
    {
        type Output = Prod<Factor, UInt<U, B>>;
    }

    impl<Factor, U, B> GcdWithFactor<UTerm, Factor> for UInt<U, B>
    where
        U: Unsigned,
        B: Bit,
        Factor: Unsigned + Mul<UInt<U, B>>,
        Prod<Factor, UInt<U, B>>: Unsigned,
    {
        type Output = Prod<Factor, UInt<U, B>>;
    }

    impl<UL, UR, Factor> GcdWithFactor<UInt<UR, B0>, Factor> for UInt<UL, B0>
    where
        UL: Unsigned + GcdWithFactor<UR, Prod<Factor, U2>>,
        UR: Unsigned,
        Factor: Unsigned + Mul<U2>,
        Prod<Factor, U2>: Unsigned,
    {
        type Output = op_aliases::GcdWithFactor<UL, UR, Prod<Factor, U2>>;
    }

    impl<UL, UR, Factor> GcdWithFactor<UInt<UR, B0>, Factor> for UInt<UL, B1>
    where
        Self: GcdWithFactor<UR, Factor>,
        UL: Unsigned,
        UR: Unsigned,
        Factor: Unsigned,
    {
        type Output = op_aliases::GcdWithFactor<Self, UR, Factor>;
    }

    impl<UL, UR, Factor> GcdWithFactor<UInt<UR, B1>, Factor> for UInt<UL, B0>
    where
        UL: Unsigned + GcdWithFactor<UInt<UR, B1>, Factor>,
        UR: Unsigned,
        Factor: Unsigned,
    {
        type Output = op_aliases::GcdWithFactor<UL, UInt<UR, B1>, Factor>;
    }

    impl<UL, UR, Factor> GcdWithFactor<UInt<UR, B1>, Factor> for UInt<UL, B1>
    where
        UL: Unsigned + SaturatingSub<UR>,
        UR: Unsigned + SaturatingSub<UL>,
        Factor: Unsigned + Mul<UInt<UL, B1>>,
        op_aliases::SaturatingSub<UL, UR>: GcdWithFactor<UInt<UR, B1>, Factor>,
        op_aliases::SaturatingSub<UR, UL>: GcdWithFactor<UInt<UL, B1>, Factor>,
        (): control::ops::IfElseGreater<
                UInt<UL, B1>,
                UInt<UR, B1>,
                op_aliases::GcdWithFactor<op_aliases::SaturatingSub<UL, UR>, UInt<UR, B1>, Factor>,
                op_aliases::GcdWithFactor<op_aliases::SaturatingSub<UR, UL>, UInt<UL, B1>, Factor>,
            > + control::ops::IfElseEqual<
                UInt<UL, B1>,
                UInt<UR, B1>,
                Prod<Factor, UInt<UL, B1>>,
                control::op_aliases::IfElseGreater<
                    UInt<UL, B1>,
                    UInt<UR, B1>,
                    op_aliases::GcdWithFactor<
                        op_aliases::SaturatingSub<UL, UR>,
                        UInt<UR, B1>,
                        Factor,
                    >,
                    op_aliases::GcdWithFactor<
                        op_aliases::SaturatingSub<UR, UL>,
                        UInt<UL, B1>,
                        Factor,
                    >,
                >,
            >,
        control::op_aliases::IfElseEqual<
            UInt<UL, B1>,
            UInt<UR, B1>,
            Prod<Factor, UInt<UL, B1>>,
            control::op_aliases::IfElseGreater<
                UInt<UL, B1>,
                UInt<UR, B1>,
                op_aliases::GcdWithFactor<op_aliases::SaturatingSub<UL, UR>, UInt<UR, B1>, Factor>,
                op_aliases::GcdWithFactor<op_aliases::SaturatingSub<UR, UL>, UInt<UL, B1>, Factor>,
            >,
        >: Unsigned,
    {
        type Output = control::op_aliases::IfElseEqual<
            UInt<UL, B1>,               // lhs
            UInt<UR, B1>,               // rhs
            Prod<Factor, UInt<UL, B1>>, // equal output
            // non-equal output
            control::op_aliases::IfElseGreater<
                UInt<UL, B1>, // lhs
                UInt<UR, B1>, // rhs
                // greater output
                op_aliases::GcdWithFactor<op_aliases::SaturatingSub<UL, UR>, UInt<UR, B1>, Factor>,
                // less output
                op_aliases::GcdWithFactor<op_aliases::SaturatingSub<UR, UL>, UInt<UL, B1>, Factor>,
            >,
        >;
    }

    // popcount

    pub trait PopCount
    where
        Self: Unsigned,
        Self::Output: Unsigned,
    {
        type Output;
    }

    impl<Input> PopCount for Input
    where
        Input: Unsigned + PopCountWithBase<UTerm>,
    {
        type Output = op_aliases::PopCountWithBase<Input, UTerm>;
    }

    // popcount with base

    pub trait PopCountWithBase<Base>
    where
        Self: Unsigned,
        Self::Output: Unsigned,
        Base: Unsigned,
    {
        type Output;
    }

    impl<Base> PopCountWithBase<Base> for UTerm
    where
        Base: Unsigned,
    {
        type Output = Base;
    }

    impl<Base, U> PopCountWithBase<Base> for UInt<U, B0>
    where
        U: Unsigned + PopCountWithBase<Base>,
        Base: Unsigned,
    {
        type Output = op_aliases::PopCountWithBase<U, Base>;
    }

    impl<Base, U> PopCountWithBase<Base> for UInt<U, B1>
    where
        U: Unsigned + PopCountWithBase<Add1<Base>>,
        Base: Unsigned + Add<B1>,
        Add1<Base>: Unsigned,
    {
        type Output = op_aliases::PopCountWithBase<U, Add1<Base>>;
    }

    // binary to sequence of bits

    pub trait BitSeqOf
    where
        Self: Unsigned,
        Self::Output: Unsigned,
    {
        type Output;
    }

    impl<Input> BitSeqOf for Input
    where
        Input: Unsigned + BitSeqOfWithBase<UTerm>,
    {
        type Output = op_aliases::BitSeqOfWithBase<Input, UTerm>;
    }

    // binary to sequence of bits with base

    pub trait BitSeqOfWithBase<Base>
    where
        Self: Unsigned,
        Self::Output: Unsigned,
        Base: Unsigned,
    {
        type Output;
    }

    impl<Base> BitSeqOfWithBase<Base> for UTerm
    where
        Base: Unsigned,
    {
        type Output = Base;
    }

    impl<Base> BitSeqOfWithBase<Base> for UInt<UTerm, B1>
    where
        Base: Unsigned + Add<B1>,
    {
        type Output = op_aliases::BitSeqOfWithBase<UTerm, UInt<Base, B1>>;
    }

    impl<Base, U, B> BitSeqOfWithBase<Base> for UInt<UInt<U, B>, B1>
    where
        UInt<UInt<U, B>, B0>: BitSeqOfWithBase<UInt<Base, B1>>,
        Base: Unsigned,
        U: Unsigned,
        B: Bit,
    {
        type Output = op_aliases::BitSeqOfWithBase<UInt<UInt<U, B>, B0>, UInt<Base, B1>>;
    }

    impl<Base, U> BitSeqOfWithBase<Base> for UInt<U, B0>
    where
        Base: Unsigned,
        U: Unsigned + Sub<B1>,
        UInt<Sub1<U>, B1>: BitSeqOfWithBase<UInt<Base, B1>>,
    {
        type Output = op_aliases::BitSeqOfWithBase<UInt<Sub1<U>, B1>, UInt<Base, B1>>;
    }

}

pub mod op_aliases {
    use super::*;

    pub type PopCount<Input> = <Input as ops::PopCount>::Output;
    pub type PopCountWithBase<Input, Base> = <Input as ops::PopCountWithBase<Base>>::Output;
    pub type BitSeqOf<Input> = <Input as ops::BitSeqOf>::Output;
    pub type BitSeqOfWithBase<Input, Base> = <Input as ops::BitSeqOfWithBase<Base>>::Output;
    pub type Gcd<Lhs, Rhs> = <Lhs as ops::Gcd<Rhs>>::Output;
    pub type GcdWithFactor<Lhs, Rhs, Factor> = <Lhs as ops::GcdWithFactor<Rhs, Factor>>::Output;
    pub type SaturatingSub<Lhs, Rhs> = <Lhs as ops::SaturatingSub<Rhs>>::Output;
}

#[cfg(test)]
mod tests {
    use super::op_aliases::*;
    use crate::control::op_aliases::*;
    use typenum::{U0, U1, U126, U15, U2, U21, U3, U4, U42, U5, U6, U7, U84};

    type Assert1 = AssertSame<PopCount<U0>, U0, ()>;
    type Assert2 = AssertSame<PopCount<U1>, U1, ()>;
    type Assert3 = AssertSame<PopCount<U2>, U1, ()>;
    type Assert4 = AssertSame<PopCount<U3>, U2, ()>;
    type Assert5 = AssertSame<BitSeqOf<U0>, U0, ()>;
    type Assert6 = AssertSame<BitSeqOf<U1>, U1, ()>;
    type Assert7 = AssertSame<BitSeqOf<U2>, U3, ()>;
    type Assert8 = AssertSame<BitSeqOf<U3>, U7, ()>;
    type Assert9 = AssertSame<BitSeqOf<U4>, U15, ()>;
    type Assert10 = AssertSame<SaturatingSub<U5, U0>, U5, ()>;
    type Assert11 = AssertSame<SaturatingSub<U5, U1>, U4, ()>;
    type Assert12 = AssertSame<SaturatingSub<U5, U2>, U3, ()>;
    type Assert13 = AssertSame<SaturatingSub<U5, U3>, U2, ()>;
    type Assert14 = AssertSame<SaturatingSub<U5, U4>, U1, ()>;
    type Assert15 = AssertSame<SaturatingSub<U5, U5>, U0, ()>;
    type Assert16 = AssertSame<SaturatingSub<U5, U6>, U0, ()>;
    type Assert17 = AssertSame<SaturatingSub<U5, U7>, U0, ()>;
    type Assert18 = AssertSame<SaturatingSub<U0, U0>, U0, ()>;
    type Assert19 = AssertSame<SaturatingSub<U0, U1>, U0, ()>;
    type Assert20 = AssertSame<SaturatingSub<U0, U2>, U0, ()>;
    type Assert21 = AssertSame<SaturatingSub<U0, U3>, U0, ()>;
    type Assert22 = AssertSame<Gcd<U0, U3>, U3, ()>;
    type Assert23 = AssertSame<Gcd<U1, U3>, U1, ()>;
    type Assert24 = AssertSame<Gcd<U2, U3>, U1, ()>;
    type Assert25 = AssertSame<Gcd<U3, U3>, U3, ()>;
    type Assert26 = AssertSame<Gcd<U3, U2>, U1, ()>;
    type Assert27 = AssertSame<Gcd<U3, U1>, U1, ()>;
    type Assert28 = AssertSame<Gcd<U3, U0>, U3, ()>;
    type Assert29 = AssertSame<Gcd<U21, U6>, U3, ()>;
    type Assert30 = AssertSame<Gcd<U0, U4>, U4, ()>;
    type Assert31 = AssertSame<Gcd<U1, U4>, U1, ()>;
    type Assert32 = AssertSame<Gcd<U2, U4>, U2, ()>;
    type Assert33 = AssertSame<Gcd<U3, U4>, U1, ()>;
    type Assert34 = AssertSame<Gcd<U4, U4>, U4, ()>;
    type Assert35 = AssertSame<Gcd<U4, U3>, U1, ()>;
    type Assert36 = AssertSame<Gcd<U4, U2>, U2, ()>;
    type Assert37 = AssertSame<Gcd<U4, U1>, U1, ()>;
    type Assert38 = AssertSame<Gcd<U4, U0>, U4, ()>;
    type Assert39 = AssertSame<Gcd<U1, U1>, U1, ()>;
    type Assert40 = AssertSame<Gcd<U84, U126>, U42, ()>;

    #[test]
    fn numeric_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
        let _: Assert9 = ();
        let _: Assert10 = ();
        let _: Assert11 = ();
        let _: Assert12 = ();
        let _: Assert13 = ();
        let _: Assert14 = ();
        let _: Assert15 = ();
        let _: Assert16 = ();
        let _: Assert17 = ();
        let _: Assert18 = ();
        let _: Assert19 = ();
        let _: Assert20 = ();
        let _: Assert21 = ();
        let _: Assert22 = ();
        let _: Assert23 = ();
        let _: Assert24 = ();
        let _: Assert25 = ();
        let _: Assert26 = ();
        let _: Assert27 = ();
        let _: Assert28 = ();
        let _: Assert29 = ();
        let _: Assert30 = ();
        let _: Assert31 = ();
        let _: Assert32 = ();
        let _: Assert33 = ();
        let _: Assert34 = ();
        let _: Assert35 = ();
        let _: Assert36 = ();
        let _: Assert37 = ();
        let _: Assert38 = ();
        let _: Assert39 = ();
        let _: Assert40 = ();
    }
}
