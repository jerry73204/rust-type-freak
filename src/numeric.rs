//! Numeric type operators and functors.

use crate::common::*;

pub use ops::*;

pub mod ops {
    use super::*;

    typ! {
        pub fn Gcd<lhs, rhs>(lhs: Unsigned, rhs: Unsigned) -> Unsigned {
            if lhs == rhs {
                lhs
            } else if lhs == 0u {
                rhs
            } else if rhs == 0u {
                lhs
            } else {
                if lhs % 2u == 1u {
                    if rhs % 2u == 1u {
                        if lhs > rhs {
                            Gcd(lhs - rhs, rhs)
                        } else {
                            Gcd(rhs - lhs, lhs)
                        }
                    } else {
                        Gcd(lhs, rhs / 2u)

                    }
                } else {
                    if rhs % 2u == 1u {
                        Gcd(lhs / 2u, rhs)
                    } else {
                        Gcd(lhs / 2u, rhs / 2u) * 2u
                    }
                }
            }
        }

        pub fn Lcm<lhs, rhs>(lhs: Unsigned, rhs: Unsigned) -> Unsigned {
            lhs * rhs / Gcd(lhs, rhs)
        }

        pub fn PopCount<value>(value: Unsigned) -> Unsigned {
            match value {
                UTerm => 0u,
                #[generics(U: Unsigned)]
                UInt::<U, B0> => PopCount(U),
                #[generics(U: Unsigned)]
                UInt::<U, B1> => PopCount(U) + 1u,
            }
        }
    }

    // to uint

    // pub trait ToInt<T> {
    //     fn to_int(&self) -> T;
    // }

    // impl ToInt<usize> for UTerm {
    //     fn to_int(&self) -> usize {
    //         Self::USIZE
    //     }
    // }

    // impl<U, B> ToInt<usize> for UInt<U, B>
    // where
    //     U: Unsigned,
    //     B: Bit,
    // {
    //     fn to_int(&self) -> usize {
    //         Self::USIZE
    //     }
    // }

    // impl ToInt<usize> for DynU<usize> {
    //     fn to_int(&self) -> usize {
    //         self.0
    //     }
    // }

    // saturating subtraction

    // pub trait SaturatingSub<Rhs>
    // where
    //     Self: Unsigned,
    //     Self::Output: Unsigned,
    // {
    //     type Output;
    // }

    // impl SaturatingSub<UTerm> for UTerm {
    //     type Output = UTerm;
    // }

    // impl<U, B> SaturatingSub<UInt<U, B>> for UTerm
    // where
    //     U: Unsigned,
    //     B: Bit,
    // {
    //     type Output = UTerm;
    // }

    // impl<U, B> SaturatingSub<UTerm> for UInt<U, B>
    // where
    //     U: Unsigned,
    //     B: Bit,
    // {
    //     type Output = UInt<U, B>;
    // }

    // impl<UL, UR> SaturatingSub<UInt<UR, B1>> for UInt<UL, B1>
    // where
    //     UL: Unsigned + SaturatingSub<UR>,
    //     UR: Unsigned,
    //     (): control::ops::IfElseGreater<UL, UR, UInt<op_aliases::SaturatingSub<UL, UR>, B0>, UTerm>,
    //     control::op_aliases::IfElseGreater<
    //         UL,
    //         UR,
    //         UInt<op_aliases::SaturatingSub<UL, UR>, B0>,
    //         UTerm,
    //     >: Unsigned,
    // {
    //     type Output = control::op_aliases::IfElseGreater<
    //         UL,
    //         UR,
    //         UInt<op_aliases::SaturatingSub<UL, UR>, B0>,
    //         UTerm,
    //     >;
    // }

    // impl<UL, UR> SaturatingSub<UInt<UR, B0>> for UInt<UL, B0>
    // where
    //     UL: Unsigned + SaturatingSub<UR>,
    //     UR: Unsigned,
    //     (): control::ops::IfElseGreater<UL, UR, UInt<op_aliases::SaturatingSub<UL, UR>, B0>, UTerm>,
    //     control::op_aliases::IfElseGreater<
    //         UL,
    //         UR,
    //         UInt<op_aliases::SaturatingSub<UL, UR>, B0>,
    //         UTerm,
    //     >: Unsigned,
    // {
    //     type Output = control::op_aliases::IfElseGreater<
    //         UL,
    //         UR,
    //         UInt<op_aliases::SaturatingSub<UL, UR>, B0>,
    //         UTerm,
    //     >;
    // }

    // impl<UL, UR> SaturatingSub<UInt<UR, B0>> for UInt<UL, B1>
    // where
    //     UL: Unsigned + SaturatingSub<UR>,
    //     UR: Unsigned,
    //     (): control::ops::IfElseGreaterOrEqual<
    //         UL,
    //         UR,
    //         UInt<op_aliases::SaturatingSub<UL, UR>, B1>,
    //         UTerm,
    //     >,
    //     control::op_aliases::IfElseGreaterOrEqual<
    //         UL,
    //         UR,
    //         UInt<op_aliases::SaturatingSub<UL, UR>, B1>,
    //         UTerm,
    //     >: Unsigned,
    // {
    //     type Output = control::op_aliases::IfElseGreaterOrEqual<
    //         UL,
    //         UR,
    //         UInt<op_aliases::SaturatingSub<UL, UR>, B1>,
    //         UTerm,
    //     >;
    // }

    // impl<UL, UR> SaturatingSub<UInt<UR, B1>> for UInt<UL, B0>
    // where
    //     UL: Unsigned + Sub<B1>,
    //     UR: Unsigned,
    //     UInt<Sub1<UL>, B1>: SaturatingSub<UInt<UR, B0>>,
    // {
    //     type Output = op_aliases::SaturatingSub<UInt<Sub1<UL>, B1>, UInt<UR, B0>>;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::op_aliases::*;
    use typenum::consts::*;

    type Assert1 = AssertSame<PopCountOp<U0>, U0, ()>;
    type Assert2 = AssertSame<PopCountOp<U1>, U1, ()>;
    type Assert3 = AssertSame<PopCountOp<U2>, U1, ()>;
    type Assert4 = AssertSame<PopCountOp<U3>, U2, ()>;
    // type Assert5 = AssertSame<BitSeqOf<U0>, U0, ()>;
    // type Assert6 = AssertSame<BitSeqOf<U1>, U1, ()>;
    // type Assert7 = AssertSame<BitSeqOf<U2>, U3, ()>;
    // type Assert8 = AssertSame<BitSeqOf<U3>, U7, ()>;
    // type Assert9 = AssertSame<BitSeqOf<U4>, U15, ()>;
    // type Assert10 = AssertSame<SaturatingSub<U5, U0>, U5, ()>;
    // type Assert11 = AssertSame<SaturatingSub<U5, U1>, U4, ()>;
    // type Assert12 = AssertSame<SaturatingSub<U5, U2>, U3, ()>;
    // type Assert13 = AssertSame<SaturatingSub<U5, U3>, U2, ()>;
    // type Assert14 = AssertSame<SaturatingSub<U5, U4>, U1, ()>;
    // type Assert15 = AssertSame<SaturatingSub<U5, U5>, U0, ()>;
    // type Assert16 = AssertSame<SaturatingSub<U5, U6>, U0, ()>;
    // type Assert17 = AssertSame<SaturatingSub<U5, U7>, U0, ()>;
    // type Assert18 = AssertSame<SaturatingSub<U0, U0>, U0, ()>;
    // type Assert19 = AssertSame<SaturatingSub<U0, U1>, U0, ()>;
    // type Assert20 = AssertSame<SaturatingSub<U0, U2>, U0, ()>;
    // type Assert21 = AssertSame<SaturatingSub<U0, U3>, U0, ()>;
    type Assert22 = AssertSame<GcdOp<U0, U3>, U3, ()>;
    type Assert23 = AssertSame<GcdOp<U1, U3>, U1, ()>;
    type Assert24 = AssertSame<GcdOp<U2, U3>, U1, ()>;
    type Assert25 = AssertSame<GcdOp<U3, U3>, U3, ()>;
    type Assert26 = AssertSame<GcdOp<U3, U2>, U1, ()>;
    type Assert27 = AssertSame<GcdOp<U3, U1>, U1, ()>;
    type Assert28 = AssertSame<GcdOp<U3, U0>, U3, ()>;
    type Assert29 = AssertSame<GcdOp<U21, U6>, U3, ()>;
    type Assert30 = AssertSame<GcdOp<U0, U4>, U4, ()>;
    type Assert31 = AssertSame<GcdOp<U1, U4>, U1, ()>;
    type Assert32 = AssertSame<GcdOp<U2, U4>, U2, ()>;
    type Assert33 = AssertSame<GcdOp<U3, U4>, U1, ()>;
    type Assert34 = AssertSame<GcdOp<U4, U4>, U4, ()>;
    type Assert35 = AssertSame<GcdOp<U4, U3>, U1, ()>;
    type Assert36 = AssertSame<GcdOp<U4, U2>, U2, ()>;
    type Assert37 = AssertSame<GcdOp<U4, U1>, U1, ()>;
    type Assert38 = AssertSame<GcdOp<U4, U0>, U4, ()>;
    type Assert39 = AssertSame<GcdOp<U1, U1>, U1, ()>;
    type Assert40 = AssertSame<GcdOp<U84, U126>, U42, ()>;
    type Assert41 = AssertSame<LcmOp<U84, U126>, U252, ()>;

    #[test]
    fn numeric_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        // let _: Assert5 = ();
        // let _: Assert6 = ();
        // let _: Assert7 = ();
        // let _: Assert8 = ();
        // let _: Assert9 = ();
        // let _: Assert10 = ();
        // let _: Assert11 = ();
        // let _: Assert12 = ();
        // let _: Assert13 = ();
        // let _: Assert14 = ();
        // let _: Assert15 = ();
        // let _: Assert16 = ();
        // let _: Assert17 = ();
        // let _: Assert18 = ();
        // let _: Assert19 = ();
        // let _: Assert20 = ();
        // let _: Assert21 = ();
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
        let _: Assert41 = ();
    }
}
