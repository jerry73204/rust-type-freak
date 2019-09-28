use crate::{
    boolean::{And, AndOutput, Boolean, Or, OrOutput},
    control::{IfElsePredicate, IfElsePredicateOutput},
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

/// Composes two functors from right to left.
pub struct Compose<Lhs, Rhs> {
    _phantom: PhantomData<(Lhs, Rhs)>,
}

impl<Input, Lhs, Rhs> Functor<Input> for Compose<Lhs, Rhs>
where
    Lhs: Functor<ApplyFunctor<Rhs, Input>>,
    Rhs: Functor<Input>,
{
    type Output = ApplyFunctor<Lhs, ApplyFunctor<Rhs, Input>>;
}

/// An identity [Functor].
pub struct IdentityFunc {}

impl<Input> Functor<Input> for IdentityFunc {
    type Output = Input;
}

/// A [Functor] type that computes summation of inputs.
pub struct SumFoldFunctor;

impl<Init, Value> Functor<(Init, Value)> for SumFoldFunctor
where
    Init: Add<Value>,
{
    type Output = Sum<Init, Value>;
}

/// A [Functor] type that computes products of inputs.
pub struct ProdFoldFunctor;

impl<Init, Value> Functor<(Init, Value)> for ProdFoldFunctor
where
    Init: Mul<Value>,
{
    type Output = Prod<Init, Value>;
}

/// A [Functor] type that gets minimum of inputs.
pub struct MinFoldFunctor;

impl<Init, Value> Functor<(Init, Value)> for MinFoldFunctor
where
    Value: IsLess<Init> + IfElsePredicate<Le<Value, Init>, Init>,
    Le<Value, Init>: Boolean,
{
    type Output = IfElsePredicateOutput<Value, Le<Value, Init>, Init>;
}

/// A [Functor] type that gets maximum of inputs.
pub struct MaxFoldFunctor;

impl<Init, Value> Functor<(Init, Value)> for MaxFoldFunctor
where
    Value: IsGreater<Init> + IfElsePredicate<Gr<Value, Init>, Init>,
    Gr<Value, Init>: Boolean,
{
    type Output = IfElsePredicateOutput<Value, Gr<Value, Init>, Init>;
}

/// A [Functor] type that boolean-ands the inputs.
pub struct BooleanAndFoldFunctor;

impl<Init, Value> Functor<(Init, Value)> for BooleanAndFoldFunctor
where
    Value: Boolean,
    Init: Boolean + And<Value>,
    AndOutput<Init, Value>: Boolean,
{
    type Output = AndOutput<Init, Value>;
}

/// A [Functor] type that boolean-or the inputs.
pub struct BooleanOrFoldFunctor;

impl<Init, Value> Functor<(Init, Value)> for BooleanOrFoldFunctor
where
    Value: Boolean,
    Init: Boolean + Or<Value>,
    OrOutput<Init, Value>: Boolean,
{
    type Output = OrOutput<Init, Value>;
}
