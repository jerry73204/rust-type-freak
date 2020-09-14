use crate::list::{Cons, List};

// list

/// The trait represents a list of key-value pairs.
pub trait Dict
where
    Self: List,
{
}

/// A node of [Dict].
pub type DictCons<Key, Value, Tail> = Cons<(Key, Value), Tail>;

pub type Nil = crate::list::Nil;

impl<Key, Value, Tail> Dict for DictCons<Key, Value, Tail> where Tail: Dict {}

impl Dict for Nil {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::SameOp, Dict};

    struct Ka;
    struct Kb;
    struct Kc;
    struct Va;
    struct Vb;
    struct Vc;

    #[test]
    fn dict_test() {
        let _: SameOp<Dict! {}, Nil> = ();
        let _: SameOp<Dict! { Ka: Va }, DictCons<Ka, Va, Nil>> = ();
        let _: SameOp<Dict! { Ka: Va, Kb: Vb }, DictCons<Ka, Va, DictCons<Kb, Vb, Nil>>> = ();
        let _: SameOp<
            Dict! { Ka: Va, Kb: Vb, Kc: Vc },
            DictCons<Ka, Va, DictCons<Kb, Vb, DictCons<Kc, Vc, Nil>>>,
        > = ();
        let _: SameOp<Dict! { Ka: Va, Kb: Va }, DictCons<Ka, Va, DictCons<Kb, Va, Nil>>> = ();
    }
}
