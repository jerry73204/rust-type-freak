use crate::{
    boolean::Boolean,
    boolean::{And, AndOutput, Or, OrOutput},
    control::{IfElsePredicate, IfElsePredicateOutput},
    list::{LCons, TList},
};
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};
use typenum::{Gr, IsGreater, IsLess, Le, Prod, Sum};

/// Represents an applicable unit that takes input and produces output.
pub trait Functor<Input> {
    type Output;
}

pub type ApplyFunctor<Func, Input> = <Func as Functor<Input>>::Output;

/// Represents an applicable unit that accumulates the inputs.
pub trait FoldFunctor<Acc, Input> {
    type Output;
}

pub type ApplyFoldFunctor<Func, Acc, Input> = <Func as FoldFunctor<Acc, Input>>::Output;

/// A [Functor] type that prepends a type to a [TList].
pub struct PrependTListFunc<Tail>
where
    Tail: TList,
{
    _phantom: PhantomData<Tail>,
}

impl<Head, Tail> Functor<Head> for PrependTListFunc<Tail>
where
    Tail: TList,
{
    type Output = LCons<Head, Tail>;
}

/// A [FoldFunctor] type that computes summation of inputs.
pub struct SumFoldFunc;

impl<Init, Value> FoldFunctor<Init, Value> for SumFoldFunc
where
    Init: Add<Value>,
{
    type Output = Sum<Init, Value>;
}

/// A [FoldFunctor] type that computes products of inputs.
pub struct ProdFoldFunc;

impl<Init, Value> FoldFunctor<Init, Value> for ProdFoldFunc
where
    Init: Mul<Value>,
{
    type Output = Prod<Init, Value>;
}

/// A [FoldFunctor] type that gets minimum of inputs.
pub struct MinFoldFunc;

impl<Init, Value> FoldFunctor<Init, Value> for MinFoldFunc
where
    Value: IsLess<Init> + IfElsePredicate<Le<Value, Init>, Init>,
    Le<Value, Init>: Boolean,
{
    type Output = IfElsePredicateOutput<Value, Le<Value, Init>, Init>;
}

// /// A [FoldFunctor] type that gets maximum of inputs.
pub struct MaxFoldFunc;

impl<Init, Value> FoldFunctor<Init, Value> for MaxFoldFunc
where
    Value: IsGreater<Init> + IfElsePredicate<Gr<Value, Init>, Init>,
    Gr<Value, Init>: Boolean,
{
    type Output = IfElsePredicateOutput<Value, Gr<Value, Init>, Init>;
}

/// A [FoldFunctor] type that boolean-ands the inputs.
pub struct BooleanAndFoldFunc;

impl<Init, Value> FoldFunctor<Init, Value> for BooleanAndFoldFunc
where
    Value: Boolean,
    Init: Boolean + And<Value>,
    AndOutput<Init, Value>: Boolean,
{
    type Output = AndOutput<Init, Value>;
}

/// A [FoldFunctor] type that boolean-or the inputs.
pub struct BooleanOrFoldFunc;

impl<Init, Value> FoldFunctor<Init, Value> for BooleanOrFoldFunc
where
    Value: Boolean,
    Init: Boolean + Or<Value>,
    OrOutput<Init, Value>: Boolean,
{
    type Output = OrOutput<Init, Value>;
}

/// A [FoldFunctor] that reverses a [TList].
pub struct CollectReverseTListFoldFunc;

impl<Init, Value> FoldFunctor<Init, Value> for CollectReverseTListFoldFunc
where
    Init: TList,
{
    type Output = LCons<Value, Init>;
}
