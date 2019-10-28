use super::{LConcatComposeMap, LCons, LNil, LPrependToMap, TList};
use crate::{
    functional::{ApplicativeMap, ApplyMap, FMapMap, Map},
    maybe::{Maybe, MaybeMap, MaybeMapMap, UnwrapOr, UnwrapOrMap},
    tuple::{FirstOf, FirstOfMap, Pair, SecondOf, SecondOfMap},
};
use std::marker::PhantomData;

/// A type operator that apply a [Map] to all types in [TList].
pub trait LMapOp<Func>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LMapOpOutput<List, Func> = <List as LMapOp<Func>>::Output;

impl<Func> LMapOp<Func> for LNil {
    type Output = LNil;
}

impl<Func, Head, Tail> LMapOp<Func> for LCons<Head, Tail>
where
    Func: Map<Head>,
    Tail: TList + LMapOp<Func>,
{
    type Output = LCons<ApplyMap<Func, Head>, LMapOpOutput<Tail, Func>>;
}

/// A [Map] that maps values in [TList] with `Func`.
pub struct LMapMap<Func> {
    _phantom: PhantomData<Func>,
}

pub type LMap<List, Func> = ApplyMap<LMapMap<Func>, List>;

impl<List, Func> Map<List> for LMapMap<Func>
where
    List: TList + LMapOp<Func>,
{
    type Output = LMapOpOutput<List, Func>;
}

/// A type operator that accumulates all values in [TList].
pub trait LFoldOp<Init, Func>
where
    Self: TList,
{
    type Output;
}

pub type LFoldOpOutput<List, Init, Func> = <List as LFoldOp<Init, Func>>::Output;

impl<Init, Func> LFoldOp<Init, Func> for LNil {
    type Output = Init;
}

impl<Init, Func, Head, Tail> LFoldOp<Init, Func> for LCons<Head, Tail>
where
    Func: Map<(Init, Head)>,
    Tail: TList + LFoldOp<ApplyMap<Func, (Init, Head)>, Func>,
{
    type Output = LFoldOpOutput<Tail, ApplyMap<Func, (Init, Head)>, Func>;
}

/// A [Map] that maps values in [TList] with `Func`.
pub struct LFoldMap<Init, Func> {
    _phantom: PhantomData<(Init, Func)>,
}

pub type LFold<List, Init, Func> = ApplyMap<LFoldMap<Init, Func>, List>;

impl<List, Init, Func> Map<List> for LFoldMap<Init, Func>
where
    List: TList + LFoldOp<Init, Func>,
{
    type Output = LFoldOpOutput<List, Init, Func>;
}

/// Filters the values in [TList].
pub trait LFilterOp<Func>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LFilterOpOutput<List, Func> = <List as LFilterOp<Func>>::Output;

impl<Func> LFilterOp<Func> for LNil {
    type Output = LNil;
}

impl<Func, Head, Tail> LFilterOp<Func> for LCons<Head, Tail>
where
    Func: Map<Head>,
    Tail: TList + LFilterOp<Func>,
    Func::Output: Maybe,
    MaybeMapMap<LPrependToMap<LFilterOpOutput<Tail, Func>>>: Map<ApplyMap<Func, Head>>,
    UnwrapOrMap<LFilterOpOutput<Tail, Func>>:
        Map<MaybeMap<ApplyMap<Func, Head>, LPrependToMap<LFilterOpOutput<Tail, Func>>>>,
    UnwrapOr<
        MaybeMap<ApplyMap<Func, Head>, LPrependToMap<LFilterOpOutput<Tail, Func>>>,
        LFilterOpOutput<Tail, Func>,
    >: TList,
{
    type Output = UnwrapOr<
        MaybeMap<ApplyMap<Func, Head>, LPrependToMap<LFilterOpOutput<Tail, Func>>>,
        LFilterOpOutput<Tail, Func>,
    >;
}

/// A [Map] that filters values in [TList] with `Func`.
pub struct LFilterMap<Func> {
    _phantom: PhantomData<Func>,
}

pub type LFilter<List, Func> = ApplyMap<LFilterMap<Func>, List>;

impl<List, Func> Map<List> for LFilterMap<Func>
where
    List: TList + LFilterOp<Func>,
{
    type Output = LFilterOpOutput<List, Func>;
}

/// A [LMap]-like operator that maintains internal state.
pub trait LScanOp<State, Func>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
    type State;
}

pub type LScanOpOutput<List, State, Func> = <List as LScanOp<State, Func>>::Output;
pub type LScanOpState<List, State, Func> = <List as LScanOp<State, Func>>::State;

impl<State, Func> LScanOp<State, Func> for LNil {
    type Output = LNil;
    type State = State;
}

impl<State, Func, Head, Tail> LScanOp<State, Func> for LCons<Head, Tail>
where
    Func: Map<(State, Head)>,
    Tail: TList + LScanOp<SecondOf<ApplyMap<Func, (State, Head)>>, Func>,
    FirstOfMap: Map<ApplyMap<Func, (State, Head)>>,
    SecondOfMap: Map<ApplyMap<Func, (State, Head)>>,
    ApplyMap<Func, (State, Head)>: Pair,
{
    type Output =
        LCons<FirstOf<ApplyMap<Func, (State, Head)>>, LScanOpOutput<Tail, Self::State, Func>>;
    type State = SecondOf<ApplyMap<Func, (State, Head)>>;
}

/// A [Map] that maps values in [TList] with `Func` with internal state.
pub struct LScanMap<Init, Func> {
    _phantom: PhantomData<(Init, Func)>,
}

pub type LScan<List, Init, Func> = ApplyMap<LScanMap<Init, Func>, List>;

impl<List, Init, Func> Map<List> for LScanMap<Init, Func>
where
    List: TList + LScanOp<Init, Func>,
{
    type Output = LScanOpOutput<List, Init, Func>;
}

// impl FMap for TList

impl<Func> Map<LNil> for FMapMap<Func> {
    type Output = LNil;
}

impl<Func, Head, Tail> Map<LCons<Head, Tail>> for FMapMap<Func>
where
    Tail: TList,
    LCons<Head, Tail>: LMapOp<Func>,
{
    type Output = LMapOpOutput<LCons<Head, Tail>, Func>;
}

// impl Applicative for TList

impl Map<LNil> for ApplicativeMap<LNil> {
    type Output = LNil;
}

impl<LHead, LTail> Map<LCons<LHead, LTail>> for ApplicativeMap<LNil>
where
    LTail: TList,
{
    type Output = LNil;
}

impl<RHead, RTail> Map<LNil> for ApplicativeMap<LCons<RHead, RTail>>
where
    RTail: TList,
{
    type Output = LNil;
}

impl<LHead, LTail, RHead, RTail> Map<LCons<LHead, LTail>> for ApplicativeMap<LCons<RHead, RTail>>
where
    LTail: TList,
    RTail: TList,
    LCons<LHead, LTail>: LMapOp<ApplyToTListMap<LCons<RHead, RTail>>>,
    LMapOpOutput<LCons<LHead, LTail>, ApplyToTListMap<LCons<RHead, RTail>>>:
        LFoldOp<LNil, LConcatComposeMap>,
{
    type Output = LFoldOpOutput<
        LMapOpOutput<LCons<LHead, LTail>, ApplyToTListMap<LCons<RHead, RTail>>>,
        LNil,
        LConcatComposeMap,
    >;
}

// auxiliary map for Applicative interface

/// A [Map] that applies input map to `List`.
pub struct ApplyToTListMap<List>
where
    List: TList,
{
    _phantom: PhantomData<List>,
}

pub type ApplyToTList<Func, List> = ApplyMap<ApplyToTListMap<List>, Func>;

impl<Func, List> Map<Func> for ApplyToTListMap<List>
where
    List: TList + LMapOp<Func>,
    LMapOpOutput<List, Func>: TList,
{
    type Output = LMapOpOutput<List, Func>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        boolean::Boolean,
        control::{IfElsePredicate, IfElsePredicateOutput, IfSameOutput},
        functional::{Applicative, FMap},
        maybe::{Just, Nothing},
        numeric::{AddOneMap, SubOneMap},
        TListType,
    };
    use std::ops::Add;
    use typenum::{consts::*, Add1, Exp, IsLess, Le, Pow, Sum, Unsigned, B1};

    // Plus one to typenum unsigned numbers
    struct PlusOne;

    impl<Input> Map<Input> for PlusOne
    where
        Input: Unsigned + Add<B1>,
    {
        type Output = Add1<Input>;
    }

    type List1 = TListType![U1, U2, U3];
    type List2 = LMap<List1, PlusOne>;
    type Assert1 = IfSameOutput<(), List2, TListType![U2, U3, U4]>;

    // Box every type
    struct BoxFunc;

    impl<Input> Map<Input> for BoxFunc {
        type Output = Box<Input>;
    }

    type List3 = TListType![String, [i64; 7], isize, (), (f64, f32)];
    type List4 = LMap<List3, BoxFunc>;
    type Assert2 = IfSameOutput<
        (),
        List4,
        TListType! {
            Box<String>,
            Box<[i64; 7]>,
            Box<isize>,
            Box<()>,
            Box<(f64, f32)>
        },
    >;

    // Sum of list
    struct SumFunc;

    impl<Init, Input> Map<(Init, Input)> for SumFunc
    where
        Init: Unsigned + Add<Input>,
        Input: Unsigned,
    {
        type Output = Sum<Init, Input>;
    }

    type List5 = TListType![U3, U5, U7];
    type SumOutcome = LFold<List5, U0, SumFunc>;
    type Assert3 = IfSameOutput<(), SumOutcome, U15>;

    // Count # of elements in list
    struct CountFunc;

    impl<Init, Input> Map<(Init, Input)> for CountFunc
    where
        Init: Unsigned + Add<B1>,
    {
        type Output = Add1<Init>;
    }

    type List6 = TListType![u8, u16, u32, u64, i8, i16, i32, i64, f32, f64];
    type CountOutcome = LFold<List6, U0, CountFunc>;
    type Assert4 = IfSameOutput<(), CountOutcome, U10>;

    // Filter by threshold
    struct ThresholdFunc;

    impl<Input> Map<Input> for ThresholdFunc
    where
        Input: Unsigned + IsLess<U5>,
        Le<Input, U5>: Boolean,
        Just<Input>: IfElsePredicate<Le<Input, U5>, Nothing>,
    {
        type Output = IfElsePredicateOutput<Just<Input>, Le<Input, U5>, Nothing>;
    }

    type List7 = TListType![U8, U4, U0, U6, U9];
    type ThresholdOutcome = LFilter<List7, ThresholdFunc>;
    type Assert5 = IfSameOutput<(), ThresholdOutcome, TListType![U4, U0]>;

    // Power of values
    struct PowerScanFunc;

    impl<State, Input> Map<(State, Input)> for PowerScanFunc
    where
        Input: Unsigned + Pow<State>,
        State: Unsigned + Add<B1>,
    {
        type Output = (Exp<Input, State>, Add1<State>);
    }

    type List8 = TListType![U3, U2, U7, U0, U5];
    type PowerOutput = LScan<List8, U0, PowerScanFunc>;
    type Assert6 = IfSameOutput<(), PowerOutput, TListType![U1, U2, U49, U0, U625]>;

    // FMap interface
    type Assert7 =
        IfSameOutput<(), FMap<TListType![U1, U2, U3], AddOneMap>, TListType![U2, U3, U4]>;

    // Applicative interface
    type List9 = TListType![AddOneMap, SubOneMap];
    type List10 = TListType![U1, U2, U3];

    type Assert8 = IfSameOutput<(), Applicative<LNil, LNil>, LNil>;
    type Assert9 = IfSameOutput<(), Applicative<List9, LNil>, LNil>;
    type Assert10 = IfSameOutput<(), Applicative<LNil, List10>, LNil>;
    type Assert11 =
        IfSameOutput<(), Applicative<List9, List10>, TListType![U2, U3, U4, U0, U1, U2]>;

    #[test]
    fn list_functional_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
        let _: Assert9 = ();
        let _: Assert10 = ();
        let _: Assert11 = ();
    }
}
