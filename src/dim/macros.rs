use crate::common::*;

#[macro_export]
macro_rules! Dims {
    [] => {
        $crate::list::base::Nil
    };
    [?] => {
        $crate::dim::DynDimensions
    };
    [$dim:literal $(, $remaining:tt)* $(,)?] => {
        $crate::list::base::Cons<tyuint!($dim), $crate::Dims![$($remaining),*]>
    };
    [_ $(, $remaining:tt)* $(,)?] => {
        $crate::list::base::Cons<$crate::dim::Dyn, $crate::Dims![$($remaining),*]>
    };
}

// TODO: dynamic dim
#[macro_export]
macro_rules! dims {
    [] => {
        $crate::list::base::Nil
    };
    [$dim:literal $(, $remaining:tt)* $(,)?] => {
        $crate::list::base::Cons { head: tyuint!($dim), tail: $crate::dims![$($remaining),*] }
    };
}
