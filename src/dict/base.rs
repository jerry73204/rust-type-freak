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
    use crate::{control::op_aliases::*, Dict};

    struct Ka;
    struct Kb;
    struct Kc;
    struct Va;
    struct Vb;
    struct Vc;

    type Assert1 = AssertSame<Dict! {}, Nil, ()>;
    type Assert2 = AssertSame<Dict! {Ka: Va}, DictCons<Ka, Va, Nil>, ()>;
    type Assert3 = AssertSame<Dict! {Ka: Va, Kb: Vb}, DictCons<Ka, Va, DictCons<Kb, Vb, Nil>>, ()>;
    type Assert4 = AssertSame<
        Dict! {Ka: Va, Kb: Vb, Kc: Vc},
        DictCons<Ka, Va, DictCons<Kb, Vb, DictCons<Kc, Vc, Nil>>>,
        (),
    >;
    type Assert5 = AssertSame<Dict! {Ka: Va, Kb: Va}, DictCons<Ka, Va, DictCons<Kb, Va, Nil>>, ()>;

    #[test]
    fn dict_type_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
    }
}
