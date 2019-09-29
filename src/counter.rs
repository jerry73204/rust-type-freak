//! An counter trait that can be automatically inferred, usually used for traversal.
//!
//! ## Overview
//! The [Counter](crate::counter::Counter) trait is a handy tool to construct recursive
//! type operators. As usual, the [Current](crate::counter::Current) indicates the termination
//! step of recursion, while [Next](crate::counter::Next) indicates a recursion step.
//!
//! ## Example Usage
//! An example application is to define the [LRemoveAt](crate::list::LRemoveAt) type operator,
//! which removes a specific type from [TList](crate::list::TList). The straightforward way
//! is to distinguish two kinds of recursion steps: _found_ and _not found_ steps. For example:
//!
//! ```ignore
//! use type_freak::list::{TList, LCons, LNil};
//!
//! pub trait LRemoveAt<Target>
//! where
//!     Self: TList,
//!     Self::Output: TList,
//! {
//!     type Output;
//! }
//!
//! impl<Target, Tail> LRemoveAt<Target> for LCons<Target, Tail>  // 'found' case
//! where
//!     Tail: TList,
//! {
//!     type Output = Tail;
//! }
//!
//! impl<Target, NonTarget, Tail> LRemoveAt<Target> for LCons<NonTarget, Tail>  // 'not found' case
//! where
//!     Tail: TList + LRemoveAt<Target>,
//! {
//!     type Output = LCons<NonTarget, <Tail as LRemoveAt<Target>>::Output>;
//! }
//! ```
//!
//! However, the compiler will complains about conflicting implementations because
//! both `impl` block have the same signature. We can introduce [Counter](crate::counter::Counter)
//! to make two signature distinguishalbe.
//!
//! ```rust
//! use type_freak::{
//!     list::{TList, LCons, LNil},
//!     counter::{Counter, Current, Next},
//! };
//!
//! pub trait LRemoveAt<Target, Index>
//! where
//!     Index: Counter,
//!     Self: TList,
//!     Self::Output: TList,
//! {
//!     type Output;
//! }
//!
//! // termination step
//! impl<Target, Tail> LRemoveAt<Target, Current> for LCons<Target, Tail>
//! where
//!     Tail: TList,
//! {
//!     type Output = Tail;
//! }
//!
//! // recursion step
//! impl<Target, Index, NonTarget, Tail> LRemoveAt<Target, Next<Index>> for LCons<NonTarget, Tail>
//! where
//!     Index: Counter,
//!     Tail: TList + LRemoveAt<Target, Index>,
//! {
//!     type Output = LCons<NonTarget, <Tail as LRemoveAt<Target, Index>>::Output>;
//! }
//! ```

use crate::functional::{ApplyFunctor, Functor};
use std::{marker::PhantomData, ops::Add};
use typenum::{Sum, Unsigned, U0, U1};

/// A trait that counts the number of steps.
pub trait Counter {}

/// Represents one more step of [Counter].
pub struct Next<Count>
where
    Count: Counter,
{
    _phantom: PhantomData<Count>,
}

impl<Cnt> Counter for Next<Cnt> where Cnt: Counter {}

/// End of [Counter].
pub struct Current;

impl Counter for Current {}

/// Duplicates the [Counter].
pub struct Branch<LeftCount, RightCount>
where
    LeftCount: Counter,
    RightCount: Counter,
{
    _phantom: PhantomData<(LeftCount, RightCount)>,
}

impl<LeftCount, RightCount> Counter for Branch<LeftCount, RightCount>
where
    LeftCount: Counter,
    RightCount: Counter,
{
}

// count op

/// A type operator that counts the steps of [Counter].
pub trait CountOp
where
    Self::Output: Unsigned,
{
    type Output;
}

impl CountOp for Current {
    type Output = U0;
}

impl<Cnt> CountOp for Next<Cnt>
where
    Cnt: Counter + CountOp,
    CountOpOutput<Cnt>: Add<U1>,
    Sum<CountOpOutput<Cnt>, U1>: Unsigned,
{
    type Output = Sum<CountOpOutput<Cnt>, U1>;
}

pub type CountOpOutput<Cnt> = <Cnt as CountOp>::Output;

/// A [Functor] that counts the number of steps of [Counter].
pub struct CountFunctor;

pub type Count<Input> = ApplyFunctor<CountFunctor, Input>;

impl<Input> Functor<Input> for CountFunctor
where
    Input: Counter + CountOp,
{
    type Output = CountOpOutput<Input>;
}
