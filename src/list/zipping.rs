use super::{LCons, LNil, TList};

/// Zips two [TList]s into single [TList] of tuple pairs.
pub trait LZip<Rhs>
where
    Rhs: TList,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

type LZipOutput<Lhs, Rhs> = <Lhs as LZip<Rhs>>::Output;

impl<LHead, LTail, RHead, RTail> LZip<LCons<RHead, RTail>> for LCons<LHead, LTail>
where
    LTail: TList + LZip<RTail>,
    RTail: TList,
{
    type Output = LCons<(LHead, RHead), LZipOutput<LTail, RTail>>;
}

impl<Head, Tail> LZip<LNil> for LCons<Head, Tail>
where
    Tail: TList,
{
    type Output = LNil;
}

impl<Head, Tail> LZip<LCons<Head, Tail>> for LNil
where
    Tail: TList,
{
    type Output = LNil;
}

impl LZip<LNil> for LNil {
    type Output = LNil;
}

/// Unzip a [TList] of tuple pairs to two [TList]s.
pub trait LUnzip
where
    Self: TList,
    Self::FormerOutput: TList,
    Self::LatterOutput: TList,
{
    type FormerOutput;
    type LatterOutput;
}

type LUnzipFormerOutput<List> = <List as LUnzip>::FormerOutput;
type LUnzipLatterOutput<List> = <List as LUnzip>::LatterOutput;

impl<LHead, RHead, Tail> LUnzip for LCons<(LHead, RHead), Tail>
where
    Tail: TList + LUnzip,
{
    type FormerOutput = LCons<LHead, LUnzipFormerOutput<Tail>>;
    type LatterOutput = LCons<RHead, LUnzipLatterOutput<Tail>>;
}

impl LUnzip for LNil {
    type FormerOutput = LNil;
    type LatterOutput = LNil;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, TListType};

    type List1 = TListType! {u8, u16, u32, u64};
    type List2 = TListType! {f32, f64};
    type List3 = TListType! {(i8, u8), (i16, u16), (i32, u32)};

    type Assert1 = IfSameOutput<(), LZipOutput<List1, List2>, TListType! {(u8, f32), (u16, f64)}>;
    type Assert2 = IfSameOutput<(), LUnzipFormerOutput<List3>, TListType! {i8, i16, i32}>;
    type Assert3 = IfSameOutput<(), LUnzipLatterOutput<List3>, TListType! {u8, u16, u32}>;

    #[test]
    fn tlist_zipping_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
    }
}
