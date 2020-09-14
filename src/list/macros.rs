/// Builds a type that implements [List](crate::list::List).
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

/// Builds a type that implements [List](crate::list::List) with extra appending list.
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

    #[test]
    fn list_macros() {
        let _: SameOp<List![A, B, C], Cons<A, Cons<B, Cons<C, Nil>>>> = ();
        let _: SameOp<
            PrependList![D, E; List![A, B, C]],
            Cons<D, Cons<E, Cons<A, Cons<B, Cons<C, Nil>>>>>,
        > = ();

        {
            let a = A(2);
            let b = B("text".into());
            let c = C(false);
            let _: List![A, B, C] = list![A(3), B("string".into()), C(true)];
            let _: List![A, B, C] = list![a, b, c];
        }

        {
            let tail: List![A, B, C] = list![A(3), B("string".into()), C(true)];
            let _: PrependList![D, E; List![A, B, C]] = prepend_list![D([1, 2]), E::E1; tail];
        }

        {
            let tail: List![A, B, C] = list![A(3), B("string".into()), C(true)];
            let d = D([3, 4]);
            let e = E::E2;
            let _: PrependList![D, E; List![A, B, C]] = prepend_list![d, e; tail];
        }
    }
}
