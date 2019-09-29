use super::{LCons, LNil, TList};
use crate::functional::{ApplyFunctor, Functor};
use std::marker::PhantomData;

/// Zips two [TList]s into single [TList] of tuple pairs.
pub trait LZipOp<Rhs>
where
    Rhs: TList,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LZipOpOutput<Lhs, Rhs> = <Lhs as LZipOp<Rhs>>::Output;

impl<LHead, LTail, RHead, RTail> LZipOp<LCons<RHead, RTail>> for LCons<LHead, LTail>
where
    LTail: TList + LZipOp<RTail>,
    RTail: TList,
{
    type Output = LCons<(LHead, RHead), LZipOpOutput<LTail, RTail>>;
}

impl<Head, Tail> LZipOp<LNil> for LCons<Head, Tail>
where
    Tail: TList,
{
    type Output = LNil;
}

impl<Head, Tail> LZipOp<LCons<Head, Tail>> for LNil
where
    Tail: TList,
{
    type Output = LNil;
}

impl LZipOp<LNil> for LNil {
    type Output = LNil;
}

/// A [Functor] that zips `Lhs` and `Rhs` [TList]s
pub struct LZipFunctor<Rhs>
where
    Rhs: TList,
{
    _phantom: PhantomData<Rhs>,
}

pub type LZip<Lhs, Rhs> = ApplyFunctor<LZipFunctor<Rhs>, Lhs>;

impl<Lhs, Rhs> Functor<Lhs> for LZipFunctor<Rhs>
where
    Lhs: TList + LZipOp<Rhs>,
    Rhs: TList,
{
    type Output = LZipOpOutput<Lhs, Rhs>;
}

/// Unzip a [TList] of tuple pairs to two [TList]s.
pub trait LUnzipOp
where
    Self: TList,
    Self::FormerOutput: TList,
    Self::LatterOutput: TList,
{
    type FormerOutput;
    type LatterOutput;
}

pub type LUnzipOpFormerOutput<List> = <List as LUnzipOp>::FormerOutput;
pub type LUnzipOpLatterOutput<List> = <List as LUnzipOp>::LatterOutput;

impl<LHead, RHead, Tail> LUnzipOp for LCons<(LHead, RHead), Tail>
where
    Tail: TList + LUnzipOp,
{
    type FormerOutput = LCons<LHead, LUnzipOpFormerOutput<Tail>>;
    type LatterOutput = LCons<RHead, LUnzipOpLatterOutput<Tail>>;
}

impl LUnzipOp for LNil {
    type FormerOutput = LNil;
    type LatterOutput = LNil;
}

/// A [Functor] that unzips a [TList] of pairs.
pub struct LUnzipFunctor;

pub type LUnzip<List> = ApplyFunctor<LUnzipFunctor, List>;

impl<List> Functor<List> for LUnzipFunctor
where
    List: TList + LUnzipOp,
{
    type Output = (LUnzipOpFormerOutput<List>, LUnzipOpLatterOutput<List>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, TListType};

    type List1 = TListType![u8, u16, u32, u64];
    type List2 = TListType![f32, f64];
    type List3 = TListType![(i8, u8), (i16, u16), (i32, u32)];

    type Assert1 = IfSameOutput<(), LZip<List1, List2>, TListType![(u8, f32), (u16, f64)]>;
    type Assert2 =
        IfSameOutput<(), LUnzip<List3>, (TListType![i8, i16, i32], TListType![u8, u16, u32])>;

    #[test]
    fn tlist_zipping_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
    }
}
