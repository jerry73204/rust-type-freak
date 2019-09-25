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

use std::{marker::PhantomData, ops::Add};
use typenum::{Sum, Unsigned, U0, U1};

/// A trait that counts the number of steps.
pub trait Counter {}

pub struct Next<Count>
where
    Count: Counter,
{
    _phantom: PhantomData<Count>,
}

impl<Cnt> Counter for Next<Cnt> where Cnt: Counter {}

pub struct Current;

impl Counter for Current {}

// count op

pub trait Count
where
    Self::Output: Unsigned,
{
    type Output;
}

impl Count for Current {
    type Output = U0;
}

impl<Cnt> Count for Next<Cnt>
where
    Cnt: Counter + Count,
    CountOutput<Cnt>: Add<U1>,
    Sum<CountOutput<Cnt>, U1>: Unsigned,
{
    type Output = Sum<CountOutput<Cnt>, U1>;
}

pub type CountOutput<Cnt> = <Cnt as Count>::Output;
