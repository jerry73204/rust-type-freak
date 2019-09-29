/// Builds a type that implements [TList](crate::list::TList).
///
/// ```rust
/// use type_freak::TListType;
/// type List = TListType![i8, i16, i32];
/// // Same as LCons<i8, LCons<i16, LCons<i32, LNil>>>
/// ```
#[macro_export]
macro_rules! TListType {
    [] => { $crate::list::LNil };
    [$name:ty] => { $crate::list::LCons<$name, $crate::list::LNil> };
    ($name:ty, $($names:ty),+) => { $crate::list::LCons<$name, $crate::TListType![$($names),*]> };
}

/// Builds a type that implements [TList](crate::list::TList) with extra appending list.
///
/// ```rust
/// use type_freak::{TListType, TListTypeWithTail};
/// type Tail = TListType![f32, f64];
/// type List = TListTypeWithTail![i8, i16, i32; Tail];
/// // Same as LCons<i8, LCons<i16, LCons<i32, LCons<f32, LCons<f64, LNil>>>>>
/// ```
#[macro_export]
macro_rules! TListTypeWithTail {
    [$name:ty; $tail:ty] => { $crate::list::LCons<$name, $tail> };
    [$name:ty, $($names:ty),+; $tail:ty] => { $crate::list::LCons<$name, $crate::TListTypeWithTail![$($names),*; $tail]> };
}
