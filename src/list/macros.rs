/// Builds a type that implements [TList](crate::list::TList).
///
/// ```rust
/// use type_freak::TListType;
/// type List = TListType![i8, i16, i32];
/// // Same as Cons<i8, Cons<i16, Cons<i32, LNil>>>
/// ```

#[macro_export]
macro_rules! ListT {
    [] => {
        $crate::list::base::Nil
    };
    [$name:ty $(, $names:ty)* $(,)?] => {
        $crate::list::base::Cons<$name, $crate::ListT![$($names),*]>
    };
}

#[macro_export]
macro_rules! List {
    [] => {
        $crate::list::base::Nil
    };
    [$name:expr $(, $names:expr)* $(,)?] => {
        $crate::list::base::Cons($name, $crate::List![$($names),*])
    };
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
    [; $tail:ty] => {
        $tail
    };
    [$name:ty $(, $names:ty)* $(,)?; $tail:ty] => {
        $crate::list::base::Cons<$name, $crate::ListWithTailT![$($names),*; $tail]>
    };
}

#[macro_export]
macro_rules! ListWithTail {
    [; $tail:expr] => {
        $tail
    };
    [$name:expr $(, $names:expr)* $(,)?; $tail:expr] => {
        $crate::list::base::Cons($name, $crate::ListWithTail![$($names),*; $tail])
    };
}

#[cfg(test)]
mod tests {
    use crate::control::op_aliases::AssertSame;
    use crate::list::{Cons, Nil};

    struct A(usize);
    struct B(String);
    struct C(bool);
    struct D([u8; 2]);
    enum E {
        E1,
        E2,
    }

    type List1 = ListT![A, B, C];
    type List2 = ListWithTailT![D, E; List1];

    type Assert1 = AssertSame<List1, Cons<A, Cons<B, Cons<C, Nil>>>, ()>;
    type Assert2 = AssertSame<List2, Cons<D, Cons<E, Cons<A, Cons<B, Cons<C, Nil>>>>>, ()>;

    #[test]
    fn list_macros() {
        let _: Assert1 = ();
        let _: Assert2 = ();

        {
            let a = A(2);
            let b = B("text".into());
            let c = C(false);
            let _: List1 = List![A(3), B("string".into()), C(true)];
            let _: List1 = List![a, b, c];
        }

        {
            let tail: List1 = List![A(3), B("string".into()), C(true)];
            let _: List2 = ListWithTail![D([1, 2]), E::E1; tail];
        }

        {
            let tail: List1 = List![A(3), B("string".into()), C(true)];
            let d = D([3, 4]);
            let e = E::E2;
            let _: List2 = ListWithTail![d, e; tail];
        }
    }
}
