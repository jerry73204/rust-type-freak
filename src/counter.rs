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

use crate::list::base::{Cons, List, Nil};

pub trait Counter
where
    Self: List,
{
}

pub type Step<Tail> = Cons<(), Tail>;

impl<Tail> Counter for Step<Tail> where Tail: Counter {}

impl Counter for Nil {}
