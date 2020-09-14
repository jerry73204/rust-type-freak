/// Builds a type that implements [TList](crate::list::TList).
///
/// ```rust
/// use type_freak::TListype;
/// type List = TListype![i8, i16, i32];
/// // Same as Cons<i8, Cons<i16, Cons<i32, LNil>>>
/// ```

#[macro_export]
macro_rules! List {
    [] => {
        $crate::list::Nil
    };
    [$name:ty $(, $names:ty)* $(,)?] => {
        $crate::list::Cons<$name, $crate::List![$($names),*]>
    };
}

#[macro_export]
macro_rules! list {
    [] => {
        $crate::list::Nil
    };
    [$name:expr $(, $names:expr)* $(,)?] => {
        $crate::list::Cons { head: $name, tail: $crate::list![$($names),*] }
    };
}

/// Builds a type that implements [TList](crate::list::TList) with extra appending list.
///
/// ```rust
/// use type_freak::{TListype, TListypeWithTail};
/// type Tail = TListype![f32, f64];
/// type List = TListypeWithTail![i8, i16, i32; Tail];
/// // Same as Cons<i8, Cons<i16, Cons<i32, Cons<f32, Cons<f64, LNil>>>>>
/// ```
#[macro_export]
macro_rules! PrependList {
    [; $tail:ty] => {
        $tail
    };
    [$name:ty $(, $names:ty)* $(,)?; $tail:ty] => {
        $crate::list::Cons<$name, $crate::PrependList![$($names),*; $tail]>
    };
}

#[macro_export]
macro_rules! prepend_list {
    [; $tail:expr] => {
        $tail
    };
    [$name:expr $(, $names:expr)* $(,)?; $tail:expr] => {
        $crate::list::Cons { head: $name, tail: $crate::prepend_list![$($names),*; $tail] }
    };
}

#[macro_export]
macro_rules! ListFromTuple {
    () => {
        $crate::list::Nil
    };
    ($head:ty $(, $elems:ty)* $(,)?) => {
        $crate::list::Cons<$head, $crate::ListFromTuple!($($elems),*)>
    };
}

#[macro_export]
macro_rules! list_from_tuple {
    () => {
        $crate::list::Nil
    };
    ($head:expr $(, $elems:expr)* $(,)?) => {
        $crate::list::Cons { head: $head, tail: $crate::list_from_tuple!($($elems),*) }
    };
}

#[cfg(test)]
mod tests {
    use crate::control::SameOp;
    use crate::list::{Cons, Nil};

    struct A(usize);
    struct B(String);
    struct C(bool);
    struct D([u8; 2]);
    enum E {
        E1,
        E2,
    }

    type List1 = List![A, B, C];
    type List2 = PrependList![D, E; List1];

    type Assert1 = SameOp<List1, Cons<A, Cons<B, Cons<C, Nil>>>>;
    type Assert2 = SameOp<List2, Cons<D, Cons<E, Cons<A, Cons<B, Cons<C, Nil>>>>>>;

    #[test]
    fn list_macros() {
        let _: Assert1 = ();
        let _: Assert2 = ();

        {
            let a = A(2);
            let b = B("text".into());
            let c = C(false);
            let _: List1 = list![A(3), B("string".into()), C(true)];
            let _: List1 = list![a, b, c];
        }

        {
            let tail: List1 = list![A(3), B("string".into()), C(true)];
            let _: List2 = prepend_list![D([1, 2]), E::E1; tail];
        }

        {
            let tail: List1 = list![A(3), B("string".into()), C(true)];
            let d = D([3, 4]);
            let e = E::E2;
            let _: List2 = prepend_list![d, e; tail];
        }
    }
}
