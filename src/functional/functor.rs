use crate::boolean::Boolean;
use std::marker::PhantomData;

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
