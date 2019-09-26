/// Builds a type that implements [TList](crate::list::TList).
#[macro_export]
macro_rules! TListType {
    () => { $crate::list::LNil };
    ($name:ty) => { $crate::list::LCons<$name, $crate::list::LNil> };
    ($name:ty, $($names:ty),+) => { $crate::list::LCons<$name, $crate::TListType!($($names),*)> };
}

#[macro_export]
macro_rules! TListTypeWithTail {
    ($name:ty, $tail:ty) => { $crate::list::LCons<$name, $tail> };
    ($name:ty, $($names:ty),+, $tail:ty) => { $crate::list::LCons<$name, $crate::TListTypeWithTail!($($names),*, $tail)> };
}
