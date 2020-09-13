use crate::common::*;

#[macro_export]
macro_rules! Dims {
    [] => {
        $crate::list::base::Nil
    };
    [$dim:literal $(, $remaining:ty)* $(,)?] => {
        $crate::list::base::Cons<tyuint!($dim), $crate::Dims![$($remaining),*]>
    };
    [_ $(, $remaining:ty)* $(,)?] => {
        $crate::list::base::Cons<$crate::dims::Dyn, $crate::Dims![$($remaining),*]>
    };
}

#[macro_export]
macro_rules! dims {
    [] => {
        $crate::list::base::Nil
    };
    [$dim:literal $(, $remaining:literal)* $(,)?] => {
        $crate::list::base::Cons { head: tyuint!($dim), tail: $crate::dims![$($remaining),*] }
    };
}
