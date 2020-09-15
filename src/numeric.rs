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
                            let sub: Unsigned = lhs - rhs;
                            Gcd(sub, rhs)
                        } else {
                            let sub: Unsigned = rhs - lhs;
                            Gcd(sub, lhs)
                        }
                    } else {
                        let div: Unsigned = rhs / 2u;
                        Gcd(lhs, div)

                    }
                } else {
                    if rhs % 2u == 1u {
                        let div: Unsigned = lhs / 2u;
                        Gcd(div, rhs)
                    } else {
                        let ldiv: Unsigned = lhs / 2u;
                        let rdiv: Unsigned = rhs / 2u;
                        Gcd(ldiv, rdiv) * 2u
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

    #[test]
    fn numeric_test() {
        let _: SameOp<PopCountOp<U0>, U0> = ();
        let _: SameOp<PopCountOp<U1>, U1> = ();
        let _: SameOp<PopCountOp<U2>, U1> = ();
        let _: SameOp<PopCountOp<U3>, U2> = ();
        let _: SameOp<GcdOp<U0, U3>, U3> = ();
        let _: SameOp<GcdOp<U1, U3>, U1> = ();
        let _: SameOp<GcdOp<U2, U3>, U1> = ();
        let _: SameOp<GcdOp<U3, U3>, U3> = ();
        let _: SameOp<GcdOp<U3, U2>, U1> = ();
        let _: SameOp<GcdOp<U3, U1>, U1> = ();
        let _: SameOp<GcdOp<U3, U0>, U3> = ();
        let _: SameOp<GcdOp<U21, U6>, U3> = ();
        let _: SameOp<GcdOp<U0, U4>, U4> = ();
        let _: SameOp<GcdOp<U1, U4>, U1> = ();
        let _: SameOp<GcdOp<U2, U4>, U2> = ();
        let _: SameOp<GcdOp<U3, U4>, U1> = ();
        let _: SameOp<GcdOp<U4, U4>, U4> = ();
        let _: SameOp<GcdOp<U4, U3>, U1> = ();
        let _: SameOp<GcdOp<U4, U2>, U2> = ();
        let _: SameOp<GcdOp<U4, U1>, U1> = ();
        let _: SameOp<GcdOp<U4, U0>, U4> = ();
        let _: SameOp<GcdOp<U1, U1>, U1> = ();
        let _: SameOp<GcdOp<U84, U126>, U42> = ();
        let _: SameOp<LcmOp<U84, U126>, U252> = ();
    }
}
