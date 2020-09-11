use super::{Fraction, NFrac, PFrac, UFrac, UFraction};
use crate::{common::*, numeric::Gcd};

typ! {
    pub fn Reciprocal<frac>(frac: _) {
        match frac {
            #[generics(num: Unsigned + NonZero, deno: Unsigned + NonZero)]
            UFrac::<num, deno> => UFrac::<deno, num>,
            #[generics(num: Unsigned + NonZero, deno: Unsigned + NonZero)]
            NFrac::<UFrac<num, deno>> => NFrac::<UFrac<deno, num>>,
            #[generics(num: Unsigned + NonZero, deno: Unsigned + NonZero)]
            PFrac::<UFrac<num, deno>> => PFrac::<UFrac<deno, num>>,
        }
    }

    pub fn Reduce<frac>(frac: _) {
        match frac {
            #[generics(num: Unsigned, deno: Unsigned + NonZero)]
            UFrac::<num, deno> => {
                let gcd = Gcd(num, deno);
                let num: Unsigned = num / gcd;
                let deno: Unsigned + NonZero = deno / gcd;
                UFrac::<num, deno>
            }
            #[generics(num: Unsigned, deno: Unsigned + NonZero)]
            NFrac::<UFrac<num, deno>> => {
                let gcd = Gcd(num, deno);
                let num: Unsigned = num / gcd;
                let deno: Unsigned + NonZero = deno / gcd;
                NFrac::<UFrac<num, deno>>
            }
            #[generics(num: Unsigned, deno: Unsigned + NonZero)]
            PFrac::<UFrac<num, deno>> => {
                let gcd = Gcd(num, deno);
                let num: Unsigned = num / gcd;
                let deno: Unsigned + NonZero = deno / gcd;
                PFrac::<UFrac<num, deno>>
            }
        }
    }

    pub fn UFracAdd<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> UFraction
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        let num: Unsigned = ln * rd + rn * ld;
        let deno: Unsigned + NonZero = ld * rd;
        let frac = UFrac::<num, deno>;
        let frac: UFraction = Reduce(frac);
        frac
    }

    pub fn FracAdd<lhs, rhs>(lhs: Fraction, rhs: Fraction) -> Fraction
    {
        match lhs {
            #[generics(ln: Unsigned, ld: Unsigned + NonZero)]
            PFrac::<UFrac<ln, ld>> => {
                match rhs {
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    PFrac::<UFrac<rn, rd>> => {
                        let num: Unsigned = ln * rd + rn * ld;
                        let deno: Unsigned + NonZero = ld * rd;
                        Reduce(PFrac::<UFrac<num, deno>>);
                    }
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    NFrac::<UFrac<rn, rd>> => {
                        if ln * rd >= rn * ld {
                            let num: Unsigned = ln * rd - rn * ld;
                            let deno: Unsigned + NonZero = ld * rd;
                            Reduce(PFrac::<UFrac<num, deno>>);
                        } else {
                            let num: Unsigned = rn * ld - ln * rd;
                            let deno: Unsigned + NonZero = ld * rd;
                            Reduce(NFrac::<UFrac::<num, deno>>);
                        }
                    }
                }
            }
            #[generics(ln: Unsigned, ld: Unsigned + NonZero)]
            NFrac::<UFrac<ln, ld>> => {
                match rhs {
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    PFrac::<UFrac<rn, rd>> => {
                        if ln * rd >= rn * ld {
                            let num: Unsigned = ln * rd - rn * ld;
                            let deno: Unsigned + NonZero = ld * rd;
                            Reduce(NFrac::<UFrac<num, deno>>);
                        } else {
                            let num: Unsigned = rn * ld - ln * rd;
                            let deno: Unsigned + NonZero = ld * rd;
                            Reduce(PFrac::<UFrac::<num, deno>>);
                        }
                    }
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    NFrac::<UFrac<rn, rd>> => {
                        let num: Unsigned = ln * rd + rn * ld;
                        let deno: Unsigned + NonZero = ld * rd;
                        Reduce(NFrac::<UFrac<num, deno>>);
                    }
                }
            }
        }
    }

    pub fn UFracSub<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> UFraction
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        let num: Unsigned = ln * rd - rn * ld;
        let deno: Unsigned + NonZero = ld * rd;
        let frac = UFrac::<num, deno>;
        Reduce(frac)
    }

    pub fn FracSub<lhs, rhs>(lhs: Fraction, rhs: Fraction) -> Fraction
    {
        FracAdd(lhs, -rhs)
    }

    pub fn UFracMul<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> UFraction
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        let num = ln * rn;
        let deno = ld * rd;
        let gcd = Gcd(num, deno);
        let num: Unsigned = num / gcd;
        let deno: Unsigned + NonZero = deno / gcd;
        UFrac::<num, deno>
    }

    pub fn FracMul<lhs, rhs>(lhs: Fraction, rhs: Fraction) -> Fraction {
        match lhs {
            #[generics(ln: Unsigned, ld: Unsigned + NonZero)]
            PFrac::<UFrac<ln, ld>> => {
                match rhs {
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    PFrac::<UFrac<rn, rd>> => {
                        let num: Unsigned = ln * rn;
                        let deno: Unsigned + NonZero = ld * rd;
                        let frac = PFrac::<UFrac<num, deno>>;
                        Reduce(frac)
                    }
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    NFrac::<UFrac<rn, rd>> => {
                        let num: Unsigned = ln * rn;
                        let deno: Unsigned + NonZero = ld * rd;
                        let frac = NFrac::<UFrac<num, deno>>;
                        Reduce(frac)
                    }
                }
            }
            #[generics(ln: Unsigned, ld: Unsigned + NonZero)]
            NFrac::<UFrac<ln, ld>> => {
                match rhs {
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    PFrac::<UFrac<rn, rd>> => {
                        let num: Unsigned = ln * rn;
                        let deno: Unsigned + NonZero = ld * rd;
                        let frac = NFrac::<UFrac<num, deno>>;
                        Reduce(frac)
                    }
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    NFrac::<UFrac<rn, rd>> => {
                        let num: Unsigned = ln * rn;
                        let deno: Unsigned + NonZero = ld * rd;
                        let frac = PFrac::<UFrac<num, deno>>;
                        Reduce(frac)
                    }
                }
            }
        }
    }

    pub fn UFracDiv<lhs, rhs>(lhs: UFraction, rhs: UFraction) -> UFraction
    {
        let reciprocal = Reciprocal(rhs);
        UFracMul(lhs, reciprocal)
    }

    pub fn FracDiv<lhs, rhs>(lhs: Fraction, rhs: Fraction) -> Fraction
    {
        let reciprocal = Reciprocal(rhs);
        FracMul(lhs, reciprocal)
    }

    pub fn UFracIsEqual<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> Bit
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        ln * rd == rn * ld
    }

    pub fn UFracIsLess<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> Bit
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        ln * rd < rn * ld
    }

    pub fn UFracIsLessOrEqual<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> Bit
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        ln * rd <= rn * ld
    }


    pub fn UFracIsGreater<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> Bit
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        ln * rd > rn * ld
    }

    pub fn UFracIsGreaterOrEqual<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> Bit
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        ln * rd >= rn * ld
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::op_aliases::AssertSame;
    use typenum::consts::*;

    type Frac1 = UFrac<U3, U4>;
    type Frac2 = UFrac<U2, U9>;
    type Frac11 = UFrac<U9, U4>;
    // type Frac3 = NaiveMul<Frac1, Frac2>;
    // type Frac4 = NaiveDiv<Frac1, Frac11>;
    // type Frac5 = NaiveAdd<Frac1, Frac1>;
    type Frac7 = ReduceOp<UFrac<U2, U4>>;
    type Frac8 = ReduceOp<UFrac<U0, U4>>;
    type Frac9 = ReduceOp<PFrac<UFrac<U3, U9>>>;
    type Frac10 = ReciprocalOp<UFrac<U2, U3>>;

    // type Assert1 = AssertSame<Frac3, UFrac<U6, U36>, ()>;
    // type Assert2 = AssertSame<Frac4, UFrac<U12, U36>, ()>;
    // type Assert3 = AssertSame<Frac5, UFrac<U24, U16>, ()>;
    type Assert5 = AssertSame<Frac7, UFrac<U1, U2>, ()>;
    type Assert6 = AssertSame<Frac8, UFrac<U0, U1>, ()>;
    type Assert7 = AssertSame<Frac9, PFrac<UFrac<U1, U3>>, ()>;
    type Assert8 = AssertSame<Frac10, UFrac<U3, U2>, ()>;

    #[test]
    fn frac_test() {
        // let _: Assert1 = ();
        // let _: Assert2 = ();
        // let _: Assert3 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
    }
}
