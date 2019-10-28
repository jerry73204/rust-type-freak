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

/// Composes `Lhs` map with `Rhs` map. `Lhs` is applied before `Rhs`.
pub struct Compose<Lhs, Rhs> {
    _phantom: PhantomData<(Lhs, Rhs)>,
}

impl<Input, Lhs, Rhs> Map<Input> for Compose<Lhs, Rhs>
where
    Lhs: Map<Input>,
    Rhs: Map<ApplyMap<Lhs, Input>>,
{
    type Output = ApplyMap<Rhs, ApplyMap<Lhs, Input>>;
}

/// An identity [Map].
pub struct IdentityMap {}

impl<Input> Map<Input> for IdentityMap {
    type Output = Input;
}

/// A [Map] that applies `Func` to `(Lhs, input)` type.
pub struct LeftPartialMap<Lhs, Func> {
    _phantom: PhantomData<(Lhs, Func)>,
}

impl<Lhs, Input, Func> Map<Input> for LeftPartialMap<Lhs, Func>
where
    Func: Map<(Lhs, Input)>,
{
    type Output = ApplyMap<Func, (Lhs, Input)>;
}

/// A [Map] that applies `Func` to `(input, Rhs)` type.
pub struct RightPartialMap<Rhs, Func> {
    _phantom: PhantomData<(Rhs, Func)>,
}

impl<Input, Rhs, Func> Map<Input> for RightPartialMap<Rhs, Func>
where
    Func: Map<(Input, Rhs)>,
{
    type Output = ApplyMap<Func, (Input, Rhs)>;
}

/// A map that applies `Func` to input container type.
pub struct FMapMap<Func> {
    _phantom: PhantomData<Func>,
}

pub type FMap<Container, Func> = ApplyMap<FMapMap<Func>, Container>;
