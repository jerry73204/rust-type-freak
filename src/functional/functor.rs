use crate::{
    boolean::{And, AndOutput, Boolean, Or, OrOutput},
    control::{IfElsePredicate, IfElsePredicateOutput},
};
use std::{
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};
use typenum::{Add1, Gr, IsGreater, IsLess, Le, Prod, Sub1, Sum, B1};

// functor

/// Represents an applicable unit that takes input and produces output.
pub trait Functor<Input> {
    type Output;
}

pub type ApplyFunctor<Func, Input> = <Func as Functor<Input>>::Output;

// predicate functor

/// A [Functor] that outputs [Boolean].
pub trait Predicate<Input>
where
    Self: Functor<Input>,
    Self::Output: Boolean,
{
}

/// Composes two functors from `Rhs` to `Lhs`.
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
pub struct IdentityFunctor {}

impl<Input> Functor<Input> for IdentityFunctor {
    type Output = Input;
}

/// A [Functor] type that computes summation of inputs.
pub struct SumComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for SumComposeFunctor
where
    Lhs: Add<Rhs>,
{
    type Output = Sum<Lhs, Rhs>;
}

/// A [Functor] type that computes product of inputs.
pub struct ProdComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for ProdComposeFunctor
where
    Lhs: Mul<Rhs>,
{
    type Output = Prod<Lhs, Rhs>;
}

/// A [Functor] type that gets minimum of inputs.
pub struct MinComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for MinComposeFunctor
where
    Rhs: IsLess<Lhs> + IfElsePredicate<Le<Rhs, Lhs>, Lhs>,
    Le<Rhs, Lhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Rhs, Le<Rhs, Lhs>, Lhs>;
}

/// A [Functor] type that gets maximum of inputs.
pub struct MaxComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for MaxComposeFunctor
where
    Rhs: IsGreater<Lhs> + IfElsePredicate<Gr<Rhs, Lhs>, Lhs>,
    Gr<Rhs, Lhs>: Boolean,
{
    type Output = IfElsePredicateOutput<Rhs, Gr<Rhs, Lhs>, Lhs>;
}

/// A [Functor] type that meets [Boolean] inputs.
pub struct BooleanAndComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for BooleanAndComposeFunctor
where
    Rhs: Boolean,
    Lhs: Boolean + And<Rhs>,
    AndOutput<Lhs, Rhs>: Boolean,
{
    type Output = AndOutput<Lhs, Rhs>;
}

/// A [Functor] type that joins [Boolean] inputs.
pub struct BooleanOrComposeFunctor;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for BooleanOrComposeFunctor
where
    Rhs: Boolean,
    Lhs: Boolean + Or<Rhs>,
    OrOutput<Lhs, Rhs>: Boolean,
{
    type Output = OrOutput<Lhs, Rhs>;
}

/// A [Functor] that applies `Func` to `(Lhs, input)` type.
pub struct LeftComposeFunctor<Lhs, Func> {
    _phantom: PhantomData<(Lhs, Func)>,
}

impl<Lhs, Rhs, Func> Functor<Rhs> for LeftComposeFunctor<Lhs, Func>
where
    Func: Functor<(Lhs, Rhs)>,
{
    type Output = ApplyFunctor<Func, (Lhs, Rhs)>;
}

/// A [Functor] that applies `Func` to `(input, Rhs)` type.
pub struct RightComposeFunctor<Rhs, Func> {
    _phantom: PhantomData<(Rhs, Func)>,
}

impl<Lhs, Rhs, Func> Functor<Lhs> for RightComposeFunctor<Rhs, Func>
where
    Func: Functor<(Lhs, Rhs)>,
{
    type Output = ApplyFunctor<Func, (Lhs, Rhs)>;
}

/// A [Functor] that increases input [typenum] integer by one.
pub struct AddOneFunctor;

impl<Value> Functor<Value> for AddOneFunctor
where
    Value: Add<B1>,
{
    type Output = Add1<Value>;
}

/// A [Functor] that decreases input [typenum] integer by one.
pub struct SubOneFunctor;

impl<Value> Functor<Value> for SubOneFunctor
where
    Value: Sub<B1>,
{
    type Output = Sub1<Value>;
}
