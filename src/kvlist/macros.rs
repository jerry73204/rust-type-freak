/// Builds a type that implements [KVList](crate::kvlist::KVList).
///
/// ```rust
/// use type_freak::KVListType;
/// use typenum::consts::*;
/// type List = KVListType![(U0, String), (U3, usize)];
/// // Same as KVCons<U0, String, KVCons<U3, usize, KVNil>>
/// ```
#[macro_export]
macro_rules! KVListT {
    {} => { $crate::kvlist::KVNil };
    { $name:ty : $value:ty $(, $names:ty : $values:ty)* $(,)? } => {
        $crate::kvlist::KVCons<$name, $value, $crate::KVListT![$($names : $values),*]>
    };
}

#[macro_export]
macro_rules! KVList {
    {} => { $crate::list::Nil };
    { $name:expr => $value:expr $(, $names:expr => $values:expr)* $(,)? } => {
        $crate::list::Cons(($name, $value), $crate::KVList![$($names => $values),*])
    };
}

#[macro_export]
macro_rules! KVListWithTailT {
    {; $tail:ty} => {
        $tail
    };
    { $name:ty : $value:ty $(, $names:ty : $values:ty)* $(,)?; $tail:ty } => {
        $crate::kvlist::KVCons<$name, $value, $crate::KVListWithTailT![$($names : $values),*; $tail]>
    };
}

#[macro_export]
macro_rules! KVListWithTail {
    {; $tail:expr} => {
        $tail
    };
    { $name:expr => $value:expr $(, $names:expr => $values:expr)* $(,)?; $tail:expr } => {
        $crate::list::Cons(($name, $value), $crate::KVListWithTail![$($names => $values),*; $tail])
    };
}

#[cfg(test)]
mod tests {
    use crate::control::op_aliases::AssertSame;
    use crate::kvlist::{KVCons, KVNil};

    struct Ka;
    struct Kb;
    struct Kc;

    struct Va(usize);
    struct Vb(String);
    struct Vc(bool);

    type List1 = KVListT! {};
    type List2 = KVListT! {
        Ka: Va,
        Kb: Vb,
    };
    type List3 = KVListWithTailT! {
        Kc: Vc; List2
    };

    type Assert1 = AssertSame<List1, KVNil, ()>;
    type Assert2 = AssertSame<List2, KVCons<Ka, Va, KVCons<Kb, Vb, KVNil>>, ()>;
    type Assert3 = AssertSame<List3, KVCons<Kc, Vc, KVCons<Ka, Va, KVCons<Kb, Vb, KVNil>>>, ()>;

    #[test]
    fn kvlist_macros() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();

        {
            let _: List1 = KVList! {};
            let tail: List2 = KVList! {
                Ka => Va(8),
                Kb => Vb("string".into()),
            };
            let _: List3 = KVListWithTail! {
                Kc => Vc(true); tail
            };
        }
    }
}
