use crate::boolean::Boolean;
use std::marker::PhantomData;

// map

/// Represents an applicable unit that takes input and produces output.
pub trait Map<Input> {
    type Output;
}

pub type ApplyMap<Func, Input> = <Func as Map<Input>>::Output;

// predicate map

/// A [Map] that outputs [Boolean].
pub trait Predicate<Input>
where
    Self: Map<Input>,
    Self::Output: Boolean,
{
}

/// Composes two maps from `Rhs` to `Lhs`.
pub struct Compose<Lhs, Rhs> {
    _phantom: PhantomData<(Lhs, Rhs)>,
}

impl<Input, Lhs, Rhs> Map<Input> for Compose<Lhs, Rhs>
where
    Lhs: Map<ApplyMap<Rhs, Input>>,
    Rhs: Map<Input>,
{
    type Output = ApplyMap<Lhs, ApplyMap<Rhs, Input>>;
}

/// An identity [Map].
pub struct IdentityMap {}

impl<Input> Map<Input> for IdentityMap {
    type Output = Input;
}

/// A [Map] that applies `Func` to `(Lhs, input)` type.
pub struct LeftComposeMap<Lhs, Func> {
    _phantom: PhantomData<(Lhs, Func)>,
}

impl<Lhs, Rhs, Func> Map<Rhs> for LeftComposeMap<Lhs, Func>
where
    Func: Map<(Lhs, Rhs)>,
{
    type Output = ApplyMap<Func, (Lhs, Rhs)>;
}

/// A [Map] that applies `Func` to `(input, Rhs)` type.
pub struct RightComposeMap<Rhs, Func> {
    _phantom: PhantomData<(Rhs, Func)>,
}

impl<Lhs, Rhs, Func> Map<Lhs> for RightComposeMap<Rhs, Func>
where
    Func: Map<(Lhs, Rhs)>,
{
    type Output = ApplyMap<Func, (Lhs, Rhs)>;
}

/// A map that applies `Func` to input container type.
pub struct FMapMap<Func> {
    _phantom: PhantomData<Func>,
}

pub type FMap<Container, Func> = ApplyMap<FMapMap<Func>, Container>;
