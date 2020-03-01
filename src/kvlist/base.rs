use crate::list::{Cons, List, Nil};

// list

/// The trait represents a list of key-value pairs.
pub trait KVList
where
    Self: List,
{
}

/// A node of [KVList].
pub type KVCons<Key, Value, Tail> = Cons<(Key, Value), Tail>;

pub type KVNil = Nil;

impl<Key, Value, Tail> KVList for KVCons<Key, Value, Tail> where Tail: KVList {}

impl KVList for KVNil {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::op_aliases::IfSame, KVListT};

    struct Ka;
    struct Kb;
    struct Kc;
    struct Va;
    struct Vb;
    struct Vc;

    type Assert1 = IfSame<(), KVListT! {}, KVNil>;
    type Assert2 = IfSame<(), KVListT! {Ka: Va}, KVCons<Ka, Va, KVNil>>;
    type Assert3 = IfSame<(), KVListT! {Ka: Va, Kb: Vb}, KVCons<Ka, Va, KVCons<Kb, Vb, KVNil>>>;
    type Assert4 = IfSame<
        (),
        KVListT! {Ka: Va, Kb: Vb, Kc: Vc},
        KVCons<Ka, Va, KVCons<Kb, Vb, KVCons<Kc, Vc, KVNil>>>,
    >;
    type Assert5 = IfSame<(), KVListT! {Ka: Va, Kb: Va}, KVCons<Ka, Va, KVCons<Kb, Va, KVNil>>>;

    #[test]
    fn kvlist_type_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
    }
}
