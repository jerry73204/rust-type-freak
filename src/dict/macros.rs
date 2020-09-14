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
    use crate::{
        control::SameOp,
        dict::{DictCons, Nil},
    };

    struct Ka;
    struct Kb;
    struct Kc;

    struct Va(usize);
    struct Vb(String);
    struct Vc(bool);

    #[test]
    fn dict_macros() {
        let _: SameOp<Dict! {}, Nil> = ();
        let _: SameOp<Dict! { Ka: Va, Kb: Vb }, DictCons<Ka, Va, DictCons<Kb, Vb, Nil>>> = ();
        let _: SameOp<
            PrependDict! { Kc: Vc; Dict! { Ka: Va, Kb: Vb } },
            DictCons<Kc, Vc, DictCons<Ka, Va, DictCons<Kb, Vb, Nil>>>,
        > = ();

        {
            let _: Dict! {} = dict! {};
            let tail: Dict! { Ka: Va, Kb: Vb } = dict! {
                Ka => Va(8),
                Kb => Vb("string".into()),
            };
            let _: PrependDict! { Kc: Vc; Dict! { Ka: Va, Kb: Vb } } = prepend_dict! {
                Kc => Vc(true); tail
            };
        }
    }
}
