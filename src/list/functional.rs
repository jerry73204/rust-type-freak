use super::{LCons, LNil, TList};
use crate::{
    functional::{ApplyFoldFunctor, ApplyFunctor, FoldFunctor, Functor, PrependTListFunc},
    maybe::{Maybe, MaybeMap, MaybeMapOutput, UnwrapOr, UnwrapOrOutput},
};

/// A type operator that apply a [Functor] to all types in [TList].
pub trait LMap<Func>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LMapOutput<List, Func> = <List as LMap<Func>>::Output;

impl<Func> LMap<Func> for LNil {
    type Output = LNil;
}

impl<Func, Head, Tail> LMap<Func> for LCons<Head, Tail>
where
    Func: Functor<Head>,
    Tail: TList + LMap<Func>,
{
    type Output = LCons<ApplyFunctor<Func, Head>, LMapOutput<Tail, Func>>;
}

/// A type operator that accumulates all values in [TList].
pub trait LFold<Init, Func>
where
    Self: TList,
{
    type Output;
}

pub type LFoldOutput<List, Init, Func> = <List as LFold<Init, Func>>::Output;

impl<Init, Func> LFold<Init, Func> for LNil {
    type Output = Init;
}

impl<Init, Func, Head, Tail> LFold<Init, Func> for LCons<Head, Tail>
where
    Func: FoldFunctor<Init, Head>,
    Tail: TList + LFold<ApplyFoldFunctor<Func, Init, Head>, Func>,
{
    type Output = LFoldOutput<Tail, ApplyFoldFunctor<Func, Init, Head>, Func>;
}

/// Filters the values in [TList].
pub trait LFilter<Func>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

pub type LFilterOutput<List, Func> = <List as LFilter<Func>>::Output;

impl<Func> LFilter<Func> for LNil {
    type Output = LNil;
}

impl<Func, Head, Tail> LFilter<Func> for LCons<Head, Tail>
where
    Func: Functor<Head>,
    Tail: TList + LFilter<Func>,
    Func::Output: Maybe,
    ApplyFunctor<Func, Head>: MaybeMap<PrependTListFunc<LFilterOutput<Tail, Func>>>,
    MaybeMapOutput<ApplyFunctor<Func, Head>, PrependTListFunc<LFilterOutput<Tail, Func>>>:
        UnwrapOr<LFilterOutput<Tail, Func>>,
    UnwrapOrOutput<
        MaybeMapOutput<ApplyFunctor<Func, Head>, PrependTListFunc<LFilterOutput<Tail, Func>>>,
        LFilterOutput<Tail, Func>,
    >: TList,
{
    type Output = UnwrapOrOutput<
        MaybeMapOutput<ApplyFunctor<Func, Head>, PrependTListFunc<LFilterOutput<Tail, Func>>>,
        LFilterOutput<Tail, Func>,
    >;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        boolean::Boolean,
        control::{IfElsePredicate, IfElsePredicateOutput, IfSameOutput},
        maybe::{Just, Nothing},
        TListType,
    };
    use std::ops::Add;
    use typenum::{consts::*, Add1, IsLess, Le, Sum, Unsigned, B1};

    // Plus one to typenum unsigned numbers
    struct PlusOne;

    impl<Input> Functor<Input> for PlusOne
    where
        Input: Unsigned + Add<B1>,
    {
        type Output = Add1<Input>;
    }

    type List1 = TListType! {U1, U2, U3};
    type List2 = LMapOutput<List1, PlusOne>;
    type Assert1 = IfSameOutput<(), List2, TListType! {U2, U3, U4}>;

    // Box every type
    struct BoxFunc;

    impl<Input> Functor<Input> for BoxFunc {
        type Output = Box<Input>;
    }

    type List3 = TListType! {String, [i64; 7], isize, (), (f64, f32)};
    type List4 = LMapOutput<List3, BoxFunc>;
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

    impl<Init, Input> FoldFunctor<Init, Input> for SumFunc
    where
        Init: Unsigned + Add<Input>,
        Input: Unsigned,
    {
        type Output = Sum<Init, Input>;
    }

    type List5 = TListType! {U3, U5, U7};
    type SumOutcome = LFoldOutput<List5, U0, SumFunc>;
    type Assert3 = IfSameOutput<(), SumOutcome, U15>;

    // Count # of elements in list
    struct CountFunc;

    impl<Init, Input> FoldFunctor<Init, Input> for CountFunc
    where
        Init: Unsigned + Add<B1>,
    {
        type Output = Add1<Init>;
    }

    type List6 = TListType! {u8, u16, u32, u64, i8, i16, i32, i64, f32, f64};
    type CountOutcome = LFoldOutput<List6, U0, CountFunc>;
    type Assert4 = IfSameOutput<(), CountOutcome, U10>;

    // Filter by threshold
    struct ThresholdFunc;

    impl<Input> Functor<Input> for ThresholdFunc
    where
        Input: Unsigned + IsLess<U5>,
        Le<Input, U5>: Boolean,
        Just<Input>: IfElsePredicate<Le<Input, U5>, Nothing>,
    {
        type Output = IfElsePredicateOutput<Just<Input>, Le<Input, U5>, Nothing>;
    }

    type List7 = TListType! {U8, U4, U0, U6, U9};
    type ThresholdOutcome = LFilterOutput<List7, ThresholdFunc>;
    type Assert5 = IfSameOutput<(), ThresholdOutcome, TListType! {U4, U0}>;

    #[test]
    fn list_functional_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
    }
}
