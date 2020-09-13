#[macro_export]
macro_rules! Dict {
    {} => { $crate::dict::Nil };
    { $name:ty : $value:ty $(, $names:ty : $values:ty)* $(,)? } => {
        $crate::dict::DictCons<$name, $value, $crate::Dict!{ $($names : $values),* }>
    };
}

#[macro_export]
macro_rules! dict {
    {} => { $crate::list::Nil };
    { $name:expr => $value:expr $(, $names:expr => $values:expr)* $(,)? } => {
        $crate::list::Cons { head: ($name, $value), tail: $crate::dict!{ $($names => $values),* } }
    };
}

#[macro_export]
macro_rules! PrependDict {
    {; $tail:ty} => {
        $tail
    };
    { $name:ty : $value:ty $(, $names:ty : $values:ty)* $(,)?; $tail:ty } => {
        $crate::dict::DictCons<$name, $value, $crate::PrependDict![$($names : $values),*; $tail]>
    };
}

#[macro_export]
macro_rules! prepend_dict {
    {; $tail:expr} => {
        $tail
    };
    { $name:expr => $value:expr $(, $names:expr => $values:expr)* $(,)?; $tail:expr } => {
        $crate::list::Cons { head: ($name, $value), tail: $crate::prepend_dict![$($names => $values),*; $tail] }
    };
}

#[cfg(test)]
mod tests {
    use crate::control::op_aliases::AssertSame;
    use crate::dict::{DictCons, Nil};

    struct Ka;
    struct Kb;
    struct Kc;

    struct Va(usize);
    struct Vb(String);
    struct Vc(bool);

    type List1 = Dict! {};
    type List2 = Dict! {
        Ka: Va,
        Kb: Vb,
    };
    type List3 = PrependDict! {
        Kc: Vc; List2
    };

    type Assert1 = AssertSame<List1, Nil, ()>;
    type Assert2 = AssertSame<List2, DictCons<Ka, Va, DictCons<Kb, Vb, Nil>>, ()>;
    type Assert3 = AssertSame<List3, DictCons<Kc, Vc, DictCons<Ka, Va, DictCons<Kb, Vb, Nil>>>, ()>;

    #[test]
    fn dict_macros() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();

        {
            let _: List1 = dict! {};
            let tail: List2 = dict! {
                Ka => Va(8),
                Kb => Vb("string".into()),
            };
            let _: List3 = prepend_dict! {
                Kc => Vc(true); tail
            };
        }
    }
}
