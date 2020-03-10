#[macro_export]
macro_rules! UFracT {
    ($num:ty, $denom:ty) => {
        $crate::fraction::unsigned::UFrac<$num, $denom>
    };
}

#[macro_export]
macro_rules! FracT {
    ($num:ty, $denom:ty) => {
        $crate::fraction::signed::PFrac<$crate::fraction::unsigned::UFrac<$num, $denom>>
    };
    (-$num:ty, $denom:ty) => {
        $crate::fraction::signed::NFrac<$crate::fraction::unsigned::UFrac<$num, $denom>>
    };
    ($num:ty, -$denom:ty) => {
        $crate::fraction::signed::NFrac<$crate::fraction::unsigned::UFrac<$num, $denom>>
    };
    (-$num:ty, -$denom:ty) => {
        $crate::fraction::signed::PFrac<$crate::fraction::unsigned::UFrac<$num, $denom>>
    };
}
