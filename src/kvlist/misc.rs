use super::KVList;
use crate::{
    functional::{ApplyFunctor, Functor},
    list::{
        LConcatOp, LConcatOpOutput, LLengthOp, LLengthOpOutput, LReverse, LReverseFunctor,
        LSetEqualOp, LSetEqualOpOutput, LUnzipOp, LUnzipOpFormerOutput, TList,
    },
};
use std::marker::PhantomData;
use typenum::Unsigned;

// length of list

/// A functor that gets length of [KVList].
pub struct KVLengthFunctor;

impl<List> Functor<List> for KVLengthFunctor
where
    List: KVList + LLengthOp,
    LLengthOpOutput<List>: Unsigned,
{
    type Output = LLengthOpOutput<List>;
}

pub type KVLength<List> = ApplyFunctor<KVLengthFunctor, List>;

// reverse

/// A [Functor] that reverses a [KVList].
pub struct KVReverseFuntor {}

pub type KVReverse<List> = ApplyFunctor<KVReverseFuntor, List>;

impl<List> Functor<List> for KVReverseFuntor
where
    List: KVList,
    LReverse<List>: TList,
    LReverseFunctor: Functor<List>,
{
    type Output = LReverse<List>;
}

// set equal

/// A functor that compares if two [KVList]s have same set of keys.
pub struct KVSetEqualFuntor<Rhs, Indexes>
where
    Rhs: KVList,
    Indexes: TList,
{
    _phantom: PhantomData<(Rhs, Indexes)>,
}

pub type KVSetEqual<Lhs, Rhs, Indexes> = ApplyFunctor<KVSetEqualFuntor<Rhs, Indexes>, Lhs>;

impl<Lhs, Rhs, Indexes> Functor<Lhs> for KVSetEqualFuntor<Rhs, Indexes>
where
    Lhs: KVList + LUnzipOp,
    Rhs: KVList + LUnzipOp,
    Indexes: TList,
    LUnzipOpFormerOutput<Lhs>: LSetEqualOp<LUnzipOpFormerOutput<Rhs>, Indexes>,
{
    type Output = LSetEqualOpOutput<LUnzipOpFormerOutput<Lhs>, LUnzipOpFormerOutput<Rhs>, Indexes>;
}

// concatenate

/// A [Functor] that concatenates input and `Rhs` [KVList]s.
pub struct KVConcatFunctor<Rhs>
where
    Rhs: KVList,
{
    _phantom: PhantomData<Rhs>,
}

pub type KVConcat<Lhs, Rhs> = ApplyFunctor<KVConcatFunctor<Rhs>, Lhs>;

impl<Lhs, Rhs> Functor<Lhs> for KVConcatFunctor<Rhs>
where
    Lhs: KVList + LConcatOp<Rhs>,
    Rhs: KVList,
    LConcatOpOutput<Lhs, Rhs>: KVList,
{
    type Output = LConcatOpOutput<Lhs, Rhs>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, KVListType};

    type AssertEqual<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    struct Va;
    struct Vb;
    struct Vc;
    struct Vd;
    struct Ve;

    type SomeList = KVListType![(A, Va), (B, Vb), (C, Vc)];
    type AnotherList = KVListType![(D, Vd), (E, Ve)];
    // reverse list
    type Assert10 = AssertEqual<KVReverse<SomeList>, KVListType![(C, Vc), (B, Vb), (A, Va)]>;

    // assert identical set of items
    type Assert11<Idx> = KVSetEqual<SomeList, KVListType![(C, Vd), (A, Ve), (B, Vb)], Idx>;

    // concat
    type Assert12 = AssertEqual<
        KVConcat<SomeList, AnotherList>,
        KVListType![(A, Va), (B, Vb), (C, Vc), (D, Vd), (E, Ve)],
    >;

    #[test]
    fn kvlist_misc_test() {
        let _: Assert10 = ();
        let _: Assert11<_> = ();
        let _: Assert12 = ();
    }
}
