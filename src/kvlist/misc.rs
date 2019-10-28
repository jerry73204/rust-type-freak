use super::{KVCons, KVGetValueAt, KVGetValueAtMap, KVList, KVNil, KVRemoveAt, KVRemoveAtMap};
use crate::{
    counter::Counter,
    functional::{ApplyMap, Map},
    list::{
        LConcatOp, LConcatOpOutput, LCons, LLengthOp, LLengthOpOutput, LNil, LReverse, LReverseMap,
        LSetEqualOp, LSetEqualOpOutput, LUnzipOp, LUnzipOpFormerOutput, TList,
    },
};
use std::marker::PhantomData;
use typenum::Unsigned;

// length of list

/// A map that gets length of [KVList].
pub struct KVLengthMap;

impl<List> Map<List> for KVLengthMap
where
    List: KVList + LLengthOp,
    LLengthOpOutput<List>: Unsigned,
{
    type Output = LLengthOpOutput<List>;
}

pub type KVLength<List> = ApplyMap<KVLengthMap, List>;

// reverse

/// A [Map] that reverses a [KVList].
pub struct KVReverseFuntor {}

pub type KVReverse<List> = ApplyMap<KVReverseFuntor, List>;

impl<List> Map<List> for KVReverseFuntor
where
    List: KVList,
    LReverse<List>: TList,
    LReverseMap: Map<List>,
{
    type Output = LReverse<List>;
}

// set equal

/// A map that compares if two [KVList]s have same set of keys.
pub struct KVSetEqualFuntor<Rhs, Indexes>
where
    Rhs: KVList,
    Indexes: TList,
{
    _phantom: PhantomData<(Rhs, Indexes)>,
}

pub type KVSetEqual<Lhs, Rhs, Indexes> = ApplyMap<KVSetEqualFuntor<Rhs, Indexes>, Lhs>;

impl<Lhs, Rhs, Indexes> Map<Lhs> for KVSetEqualFuntor<Rhs, Indexes>
where
    Lhs: KVList + LUnzipOp,
    Rhs: KVList + LUnzipOp,
    Indexes: TList,
    LUnzipOpFormerOutput<Lhs>: LSetEqualOp<LUnzipOpFormerOutput<Rhs>, Indexes>,
{
    type Output = LSetEqualOpOutput<LUnzipOpFormerOutput<Lhs>, LUnzipOpFormerOutput<Rhs>, Indexes>;
}

// concatenate

/// A [Map] that concatenates input and `Rhs` [KVList]s.
pub struct KVConcatMap<Rhs>
where
    Rhs: KVList,
{
    _phantom: PhantomData<Rhs>,
}

pub type KVConcat<Lhs, Rhs> = ApplyMap<KVConcatMap<Rhs>, Lhs>;

impl<Lhs, Rhs> Map<Lhs> for KVConcatMap<Rhs>
where
    Lhs: KVList + LConcatOp<Rhs>,
    Rhs: KVList,
    LConcatOpOutput<Lhs, Rhs>: KVList,
{
    type Output = LConcatOpOutput<Lhs, Rhs>;
}

// permute

/// A trait that permutes the input [KVList] to the order of `Targets`.
pub trait KVPermuteOp<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: KVList,
    Self::Output: KVList,
{
    type Output;
}

pub type KVPermuteOpOutput<List, Targets, Indexes> =
    <List as KVPermuteOp<Targets, Indexes>>::Output;

impl KVPermuteOp<LNil, LNil> for KVNil {
    type Output = KVNil;
}

impl<List, Target, TargetTail, Index, IndexTail>
    KVPermuteOp<LCons<Target, TargetTail>, LCons<Index, IndexTail>> for List
where
    List: KVList,
    TargetTail: TList,
    Index: Counter,
    IndexTail: TList,
    KVGetValueAtMap<Target, Index>: Map<List>,
    KVRemoveAtMap<Target, Index>: Map<List>,
    KVPermuteMap<TargetTail, IndexTail>: Map<KVRemoveAt<List, Target, Index>>,
    KVPermute<KVRemoveAt<List, Target, Index>, TargetTail, IndexTail>: KVList,
{
    type Output = KVCons<
        Target,
        KVGetValueAt<List, Target, Index>,
        KVPermute<KVRemoveAt<List, Target, Index>, TargetTail, IndexTail>,
    >;
}

/// A [Map] that permutes the input [KVList] to the order of `Targets`.
pub struct KVPermuteMap<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
{
    _phantom: PhantomData<(Targets, Indexes)>,
}

pub type KVPermute<List, Targets, Indexes> = ApplyMap<KVPermuteMap<Targets, Indexes>, List>;

impl<List, Targets, Indexes> Map<List> for KVPermuteMap<Targets, Indexes>
where
    List: KVList + KVPermuteOp<Targets, Indexes>,
    Targets: TList,
    Indexes: TList,
{
    type Output = KVPermuteOpOutput<List, Targets, Indexes>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, KVListType, TListType};

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
    type Assert1 = AssertEqual<KVReverse<SomeList>, KVListType![(C, Vc), (B, Vb), (A, Va)]>;

    // assert identical set of items
    type Assert2<Idx> = KVSetEqual<SomeList, KVListType![(C, Vd), (A, Ve), (B, Vb)], Idx>;

    // concat
    type Assert3 = AssertEqual<
        KVConcat<SomeList, AnotherList>,
        KVListType![(A, Va), (B, Vb), (C, Vc), (D, Vd), (E, Ve)],
    >;

    // permute
    type Assert4<Idx> = AssertEqual<
        KVPermute<SomeList, TListType![B, C, A], Idx>,
        KVListType![(B, Vb), (C, Vc), (A, Va)],
    >;

    #[test]
    fn kvlist_misc_test() {
        let _: Assert1 = ();
        let _: Assert2<_> = ();
        let _: Assert3 = ();
        let _: Assert4<_> = ();
    }
}
