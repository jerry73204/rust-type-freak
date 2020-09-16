use super::{Float, Floating};
use crate::common::*;

typ! {
    pub fn Reduce<input>(input: Floating) -> Floating {
        match input {
            #[generics(base: Unsigned + NonZero, sig: Integer, exp: Integer)]
            Float::<base, sig, exp> => {
                if sig != 0 && sig % base == 0 {
                    let new_sig: Integer = sig / base;
                    let new_exp: Integer = exp + 1;
                    Reduce(Float::<base, new_sig, new_exp>)
                } else {
                    input
                }
            }
        }
    }

    pub fn FloatAdd<lhs, rhs>(lhs: Floating, rhs: Floating) -> Floating {
        match (lhs, rhs) {
            #[generics(base: Unsigned + NonZero, lsig: Integer, lexp: Integer, rsig: Integer, rexp: Integer)]
            (Float::<base, lsig, lexp>, Float::<base, rsig, rexp>) => {
                let min_exp: Integer = lexp.Min(rexp);

                let lpower = lexp - min_exp;
                let rpower = rexp - min_exp;

                let lsig = lsig * base.Pow(lpower);
                let rsig = rsig * base.Pow(rpower);

                let out_sig: Integer = lsig + rsig;
                Reduce(Float::<base, out_sig, min_exp>)
            }
        }
    }

    pub fn FloatMul<lhs, rhs>(lhs: Floating, rhs: Floating) -> Floating {
        match (lhs, rhs) {
            #[generics(base: Unsigned + NonZero, lsig: Integer, lexp: Integer, rsig: Integer, rexp: Integer)]
            (Float::<base, lsig, lexp>, Float::<base, rsig, rexp>) => {
                let sig: Integer = lsig * rsig;
                let exp: Integer = lexp + rexp;
                Reduce(Float::<base, sig, exp>)
            }
        }
    }
}
