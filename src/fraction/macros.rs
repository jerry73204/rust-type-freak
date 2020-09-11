#[macro_export]
macro_rules! UFrac {
    ($num:expr, $denom:expr) => {
        $crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>
    };
}

#[macro_export]
macro_rules! Frac {
    ($num:expr, $denom:expr) => {
        $crate::fraction::signed::PFrac<$crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>>
    };
    (-$num:expr, $denom:expr) => {
        $crate::fraction::signed::NFrac<$crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>>
    };
    ($num:expr, -$denom:expr) => {
        $crate::fraction::signed::NFrac<$crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>>
    };
    (-$num:expr, -$denom:expr) => {
        $crate::fraction::signed::PFrac<$crate::fraction::unsigned::UFrac<typ::tyuint!($num), typ::tyuint!($denom)>>
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        control::op_aliases::AssertSame,
        fraction::{NFrac, PFrac, UFrac},
    };
    use typenum::consts::*;

    #[test]
    fn frac_macros() {
        let _: AssertSame<UFrac!(3, 4), UFrac<U3, U4>, ()> = ();
    }
}
