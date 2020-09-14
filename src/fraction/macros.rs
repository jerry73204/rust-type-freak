#[macro_export]
macro_rules! UFrac {
    ($num:literal / $denom:literal) => {
        $crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>
    };
}

#[macro_export]
macro_rules! Frac {
    ($num:literal / $denom:literal) => {
        $crate::fraction::signed::PFrac<$crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>>
    };
    (~ $num:literal / $denom:literal) => {
        $crate::fraction::signed::NFrac<$crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>>
    };
    ($num:literal / ~ $denom:literal) => {
        $crate::fraction::signed::NFrac<$crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>>
    };
    (~ $num:literal / ~ $denom:literal) => {
        $crate::fraction::signed::PFrac<$crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>>
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        control::SameOp,
        fraction::{NFrac, PFrac, UFrac},
    };
    use typenum::consts::*;

    #[test]
    fn frac_macros() {
        let _: SameOp<UFrac!(3 / 4), UFrac<U3, U4>> = ();
        let _: SameOp<Frac!(3 / 4), PFrac<UFrac<U3, U4>>> = ();
        let _: SameOp<Frac!(~ 3 / 4), NFrac<UFrac<U3, U4>>> = ();
        let _: SameOp<Frac!(3 / ~ 4), NFrac<UFrac<U3, U4>>> = ();
        let _: SameOp<Frac!(~ 3 / ~ 4), PFrac<UFrac<U3, U4>>> = ();
    }
}
