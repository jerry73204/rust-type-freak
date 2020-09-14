#[macro_export]
macro_rules! Dims {
    [] => {
        $crate::list::Nil
    };
    [?] => {
        $crate::dim::DynDimensions
    };
    [$dim:literal $(, $remaining:tt)* $(,)?] => {
        $crate::list::Cons<tyuint!($dim), $crate::Dims![$($remaining),*]>
    };
    [_ $(, $remaining:tt)* $(,)?] => {
        $crate::list::Cons<$crate::dim::Dyn, $crate::Dims![$($remaining),*]>
    };
}

// TODO: dynamic dim
#[macro_export]
macro_rules! dims {
    [] => {
        $crate::list::Nil
    };
    [$dim:literal $(, $remaining:tt)* $(,)?] => {
        $crate::list::Cons { head: tyuint!($dim), tail: $crate::dims![$($remaining),*] }
    };
}
