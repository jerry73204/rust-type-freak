/// Builds a type that implements [TList](crate::list::TList).
///
/// ```rust
/// use type_freak::TListType;
/// type List = TListType![i8, i16, i32];
/// // Same as Cons<i8, Cons<i16, Cons<i32, LNil>>>
/// ```

#[macro_export]
macro_rules! ListT {
    [] => { $crate::list::base::Nil };
    [$name:ty] => { $crate::list::base::Cons<$name, $crate::list::base::Nil> };
    [$name:ty, $($names:ty),+] => { $crate::list::base::Cons<$name, $crate::ListT![$($names),*]> };
}

#[macro_export]
macro_rules! List {
    [] => { $crate::list::base::Nil };
    [$name:ident] => { $crate::list::base::Cons($name, $crate::list::base::Nil) };
    [$name:ident, $($names:ident),+] => { $crate::list::base::Cons($name, $crate::List![$($names),*]) };
}

/// Builds a type that implements [TList](crate::list::TList) with extra appending list.
///
/// ```rust
/// use type_freak::{TListType, TListTypeWithTail};
/// type Tail = TListType![f32, f64];
/// type List = TListTypeWithTail![i8, i16, i32; Tail];
/// // Same as Cons<i8, Cons<i16, Cons<i32, Cons<f32, Cons<f64, LNil>>>>>
/// ```
#[macro_export]
macro_rules! ListWithTailT {
    [$name:ty; $tail:ty] => { $crate::list::base::Cons<$name, $tail> };
    [$name:ty, $($names:ty),+; $tail:ty] => { $crate::list::base::Cons<$name, $crate::ListWithTailT![$($names),*; $tail]> };
}
