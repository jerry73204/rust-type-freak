//! Numeric type operators and functors.

use crate::common::*;

pub use ops::*;

mod ops {
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

        pub fn UnsignedIntegerDiv<lhs, rhs>(lhs: Unsigned, rhs: Unsigned + NonZero) -> Unsigned {
            (lhs - (lhs % rhs)) / rhs
        }

        pub fn SignedIntegerDiv<lhs, rhs>(lhs: Integer, rhs: Integer + NonZero) -> Integer {
            match (lhs, rhs) {
                #[generics(lint: Unsigned + NonZero, rint: Unsigned + NonZero)]
                (PInt::<lint>, PInt::<rint>) => {
                    let quot = UnsignedIntegerDiv(lint, rint);
                    match quot {
                        UTerm => UTerm,
                        #[generics(uint: Unsigned, bit: Bit)]
                        UInt::<uint, bit> => {
                            let quot: NonZero = quot;
                            PInt::<quot>
                        }
                    }
                }
                #[generics(lint: Unsigned + NonZero, rint: Unsigned + NonZero)]
                (NInt::<lint>, PInt::<rint>) => {
                    let quot = UnsignedIntegerDiv(lint, rint);
                    match quot {
                        UTerm => UTerm,
                        #[generics(uint: Unsigned, bit: Bit)]
                        UInt::<uint, bit> => {
                            let quot: NonZero = quot;
                            NInt::<quot>
                        }
                    }
                }
                #[generics(lint: Unsigned + NonZero, rint: Unsigned + NonZero)]
                (PInt::<lint>, NInt::<rint>) => {
                    let quot = UnsignedIntegerDiv(lint, rint);
                    match quot {
                        UTerm => UTerm,
                        #[generics(uint: Unsigned, bit: Bit)]
                        UInt::<uint, bit> => {
                            let quot: NonZero = quot;
                            NInt::<quot>
                        }
                    }
                }
                #[generics(lint: Unsigned + NonZero, rint: Unsigned + NonZero)]
                (NInt::<lint>, NInt::<rint>) => {
                    let quot = UnsignedIntegerDiv(lint, rint);
                    match quot {
                        UTerm => UTerm,
                        #[generics(uint: Unsigned, bit: Bit)]
                        UInt::<uint, bit> => {
                            let quot: NonZero = quot;
                            PInt::<quot>
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::SameOp;
    use typenum::consts::*;

    type Assert1 = SameOp<PopCountOp<U0>, U0>;
    type Assert2 = SameOp<PopCountOp<U1>, U1>;
    type Assert3 = SameOp<PopCountOp<U2>, U1>;
    type Assert4 = SameOp<PopCountOp<U3>, U2>;
    type Assert22 = SameOp<GcdOp<U0, U3>, U3>;
    type Assert23 = SameOp<GcdOp<U1, U3>, U1>;
    type Assert24 = SameOp<GcdOp<U2, U3>, U1>;
    type Assert25 = SameOp<GcdOp<U3, U3>, U3>;
    type Assert26 = SameOp<GcdOp<U3, U2>, U1>;
    type Assert27 = SameOp<GcdOp<U3, U1>, U1>;
    type Assert28 = SameOp<GcdOp<U3, U0>, U3>;
    type Assert29 = SameOp<GcdOp<U21, U6>, U3>;
    type Assert30 = SameOp<GcdOp<U0, U4>, U4>;
    type Assert31 = SameOp<GcdOp<U1, U4>, U1>;
    type Assert32 = SameOp<GcdOp<U2, U4>, U2>;
    type Assert33 = SameOp<GcdOp<U3, U4>, U1>;
    type Assert34 = SameOp<GcdOp<U4, U4>, U4>;
    type Assert35 = SameOp<GcdOp<U4, U3>, U1>;
    type Assert36 = SameOp<GcdOp<U4, U2>, U2>;
    type Assert37 = SameOp<GcdOp<U4, U1>, U1>;
    type Assert38 = SameOp<GcdOp<U4, U0>, U4>;
    type Assert39 = SameOp<GcdOp<U1, U1>, U1>;
    type Assert40 = SameOp<GcdOp<U84, U126>, U42>;
    type Assert41 = SameOp<LcmOp<U84, U126>, U252>;

    #[test]
    fn numeric_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
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
