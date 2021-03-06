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
                        Reduce(PFrac::<UFrac<num, deno>>)
                    }
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    NFrac::<UFrac<rn, rd>> => {
                        if ln * rd >= rn * ld {
                            let num: Unsigned = ln * rd - rn * ld;
                            let deno: Unsigned + NonZero = ld * rd;
                            Reduce(PFrac::<UFrac<num, deno>>)
                        } else {
                            let num: Unsigned = rn * ld - ln * rd;
                            let deno: Unsigned + NonZero = ld * rd;
                            Reduce(NFrac::<UFrac::<num, deno>>)
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
                            Reduce(NFrac::<UFrac<num, deno>>)
                        } else {
                            let num: Unsigned = rn * ld - ln * rd;
                            let deno: Unsigned + NonZero = ld * rd;
                            Reduce(PFrac::<UFrac::<num, deno>>)
                        }
                    }
                    #[generics(rn: Unsigned, rd: Unsigned + NonZero)]
                    NFrac::<UFrac<rn, rd>> => {
                        let num: Unsigned = ln * rd + rn * ld;
                        let deno: Unsigned + NonZero = ld * rd;
                        Reduce(NFrac::<UFrac<num, deno>>)
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
        let neg_rhs: Fraction = -rhs;
        FracAdd(lhs, neg_rhs)
    }

    pub fn UFracMul<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction) -> UFraction
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        let num: Unsigned = ln * rn;
        let deno: Unsigned = ld * rd;
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
        let reciprocal: UFraction = Reciprocal(rhs);
        UFracMul(lhs, reciprocal)
    }

    pub fn FracDiv<lhs, rhs>(lhs: Fraction, rhs: Fraction) -> Fraction
    {
        let reciprocal: Fraction = Reciprocal(rhs);
        FracMul(lhs, reciprocal)
    }

    pub fn UFracCmp<ln, ld, rn, rd>(UFrac::<ln, ld>: UFraction, UFrac::<rn, rd>: UFraction)
    where
        ln: Unsigned,
        ld: Unsigned + NonZero,
        rn: Unsigned,
        rd: Unsigned + NonZero,
    {
        let lhs = ln * rd;
        let rhs = rn * ld;

        if lhs > rhs {
            Greater
        } else if lhs < rhs {
            Less
        } else {
            Equal
        }
    }

    pub fn FracCmp<lhs, rhs>(lhs: Fraction, rhs: Fraction)
    {
        match (lhs, rhs) {
            #[generics(lfrac: UFraction, rfrac: UFraction)]
            (PFrac::<lfrac>, PFrac::<rfrac>) => {
                UFracCmp(lfrac, rfrac)
            }
            #[generics(lfrac: UFraction, rfrac: UFraction)]
            (PFrac::<lfrac>, NFrac::<rfrac>) => {
                Greater
            }
            #[generics(lfrac: UFraction, rfrac: UFraction)]
            (NFrac::<lfrac>, PFrac::<rfrac>) => {
                Less
            }
            #[generics(lfrac: UFraction, rfrac: UFraction)]
            (NFrac::<lfrac>, NFrac::<rfrac>) => {
                let cmp = UFracCmp(lfrac, rfrac);
                match cmp {
                    Greater => Less,
                    Equal => Equal,
                    Less => Greater,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::SameOp, Frac, UFrac};

    #[test]
    fn frac_test() {
        let _: SameOp<ReduceOp<UFrac!(2 / 4)>, UFrac!(1 / 2)> = ();
        let _: SameOp<ReduceOp<UFrac!(0 / 4)>, UFrac!(0 / 1)> = ();
        let _: SameOp<ReduceOp<Frac!(3 / 9)>, Frac!(1 / 3)> = ();
        let _: SameOp<ReduceOp<Frac!(3 / ~9)>, Frac!(~1 / 3)> = ();
        let _: SameOp<ReciprocalOp<UFrac!(3 / 2)>, UFrac!(2 / 3)> = ();
        let _: SameOp<ReciprocalOp<Frac!(3 / 2)>, Frac!(2 / 3)> = ();
        let _: SameOp<ReciprocalOp<Frac!(~3 / 2)>, Frac!(~2 / 3)> = ();
        let _: SameOp<UFracAddOp<UFrac!(1 / 2), UFrac!(1 / 3)>, UFrac!(5 / 6)> = ();
        let _: SameOp<FracAddOp<Frac!(1 / 2), Frac!(1 / 3)>, Frac!(5 / 6)> = ();
        let _: SameOp<FracAddOp<Frac!(1 / 2), Frac!(~1 / 3)>, Frac!(1 / 6)> = ();
        let _: SameOp<FracAddOp<Frac!(~1 / 2), Frac!(1 / 3)>, Frac!(~1 / 6)> = ();
        let _: SameOp<FracAddOp<Frac!(~1 / 2), Frac!(~1 / 3)>, Frac!(~5 / 6)> = ();
        let _: SameOp<UFracSubOp<UFrac!(1 / 2), UFrac!(1 / 3)>, UFrac!(1 / 6)> = ();
        let _: SameOp<FracSubOp<Frac!(1 / 2), Frac!(1 / 3)>, Frac!(1 / 6)> = ();
        let _: SameOp<FracSubOp<Frac!(1 / 2), Frac!(~1 / 3)>, Frac!(5 / 6)> = ();
        let _: SameOp<FracSubOp<Frac!(~1 / 2), Frac!(1 / 3)>, Frac!(~5 / 6)> = ();
        let _: SameOp<FracSubOp<Frac!(~1 / 2), Frac!(~1 / 3)>, Frac!(~1 / 6)> = ();
        let _: SameOp<UFracMulOp<UFrac!(2 / 3), UFrac!(9 / 4)>, UFrac!(3 / 2)> = ();
        let _: SameOp<FracMulOp<Frac!(2 / 3), Frac!(9 / 4)>, Frac!(3 / 2)> = ();
        let _: SameOp<FracMulOp<Frac!(~2 / 3), Frac!(9 / 4)>, Frac!(~3 / 2)> = ();
        let _: SameOp<FracMulOp<Frac!(2 / 3), Frac!(~9 / 4)>, Frac!(~3 / 2)> = ();
        let _: SameOp<FracMulOp<Frac!(~2 / 3), Frac!(~9 / 4)>, Frac!(3 / 2)> = ();
        let _: SameOp<UFracDivOp<UFrac!(2 / 3), UFrac!(4 / 9)>, UFrac!(3 / 2)> = ();
        let _: SameOp<FracDivOp<Frac!(2 / 3), Frac!(4 / 9)>, Frac!(3 / 2)> = ();
        let _: SameOp<FracDivOp<Frac!(~2 / 3), Frac!(4 / 9)>, Frac!(~3 / 2)> = ();
        let _: SameOp<FracDivOp<Frac!(2 / 3), Frac!(~4 / 9)>, Frac!(~3 / 2)> = ();
        let _: SameOp<FracDivOp<Frac!(~2 / 3), Frac!(~4 / 9)>, Frac!(3 / 2)> = ();
        let _: SameOp<UFracCmpOp<UFrac!(1 / 3), UFrac!(1 / 2)>, Less> = ();
        let _: SameOp<UFracCmpOp<UFrac!(1 / 2), UFrac!(1 / 3)>, Greater> = ();
        let _: SameOp<UFracCmpOp<UFrac!(3 / 7), UFrac!(3 / 7)>, Equal> = ();
        let _: SameOp<UFracCmpOp<UFrac!(3 / 7), UFrac!(6 / 14)>, Equal> = ();
        let _: SameOp<FracCmpOp<Frac!(1 / 3), Frac!(1 / 2)>, Less> = ();
        let _: SameOp<FracCmpOp<Frac!(1 / 3), Frac!(~1 / 2)>, Greater> = ();
        let _: SameOp<FracCmpOp<Frac!(~1 / 3), Frac!(1 / 2)>, Less> = ();
        let _: SameOp<FracCmpOp<Frac!(~1 / 3), Frac!(~1 / 2)>, Greater> = ();
    }
}
