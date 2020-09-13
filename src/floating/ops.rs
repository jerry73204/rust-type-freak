use super::{Float, Floating};
use crate::common::*;

typ! {
    // TODO: fix missing predicates in typ
    // pub fn FloatAdd<lhs, rhs>(lhs: Floating, rhs: Floating) -> Floating {
    //     match (lhs, rhs) {
    //         #[generics(base: Unsigned + NonZero, lsig: Integer, lexp: Integer, rsig: Integer, rexp: Integer)]
    //         (Float::<base, lsig, lexp>, Float::<base, rsig, rexp>) => {
    //             let min_exp = <lexp as Min<rexp>>::Output;

    //             let lpower = lexp - min_exp;
    //             let rpower = rexp - min_exp;

    //             let lsig = lsig * <base as Pow<lpower>>::Output;
    //             let rsig = rsig * <base as Pow<rpower>>::Output;

    //             let out_sig = lsig + rsig;
    //             Float::<base, out_sig, min_exp>
    //         }
    //     }
    // }

    pub fn FloatMul<lhs, rhs>(lhs: Floating, rhs: Floating) -> Floating {
        match (lhs, rhs) {
            #[generics(base: Unsigned + NonZero, lsig: Integer, lexp: Integer, rsig: Integer, rexp: Integer)]
            (Float::<base, lsig, lexp>, Float::<base, rsig, rexp>) => {
                let sig: Integer = lsig * rsig;
                let exp: Integer = lexp + rexp;
                Float::<base, sig, exp>
            }
        }
    }
}
