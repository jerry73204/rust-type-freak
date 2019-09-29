//! Trait level boolean algebra.
//!
//! This module reuses [True](typenum::True) and [False](typenum::False) types
//! from [typenum]. It provides common logic arithmetic boolean operators:
//! [Not](crate::boolean::Not), [And](crate::boolean::And), [Or](crate::boolean::Or),
//! [Xor](crate::boolean::Xor) and [Iff](crate::boolean::Iff).
//!
//! ```rust
//! use typenum::{True, False};
//! use type_freak::boolean::*;
//!
//! type L = True;
//! type R = False;
//!
//! type R1 = Not<L>;          // False
//! type R2 = And<L, R>;       // False
//! type R3 = Or<L, R>;        // True
//! type R4 = Xor<L, R>;       // True
//! type R5 = Iff<L, R>;       // False
//! type R6 = Imply<L, R>;     // False
//! type R7 = NotImply<L, R>;  // True
//!
//! fn get_value() -> bool {
//!     R1::BOOL  // Get constant value
//! }
//! ```

use crate::functional::{ApplyFunctor, Functor, Predicate};
use std::marker::PhantomData;
use typenum::{False, True};
// boolean type def

/// A trait that provides boolean constant value.
pub trait Boolean {
    const BOOL: bool;
}

impl Boolean for False {
    const BOOL: bool = false;
}

impl Boolean for True {
    const BOOL: bool = true;
}

// and op

/// A [Predicate] that meets input and `Rhs` with [Boolean] type.
pub struct AndPredicate<Rhs>
where
    Rhs: Boolean,
{
    _phantom: PhantomData<Rhs>,
}

pub type And<Lhs, Rhs> = ApplyFunctor<AndPredicate<Rhs>, Lhs>;

impl<Lhs, Rhs> Predicate<Lhs> for AndPredicate<Rhs>
where
    Lhs: Boolean,
    Rhs: Boolean,
    AndPredicate<Rhs>: Functor<Lhs>,
    And<Lhs, Rhs>: Boolean,
{
}

impl Functor<True> for AndPredicate<True> {
    type Output = True;
}

impl Functor<False> for AndPredicate<True> {
    type Output = False;
}

impl Functor<True> for AndPredicate<False> {
    type Output = False;
}

impl Functor<False> for AndPredicate<False> {
    type Output = False;
}

/// A [Predicate] that meets input pair `(Lhs, Rhs)` with [Boolean] type.
pub struct AndComposePredicate;

pub type AndCompose<Lhs, Rhs> = ApplyFunctor<AndComposePredicate, (Lhs, Rhs)>;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for AndComposePredicate
where
    Lhs: Boolean,
    Rhs: Boolean,
    AndPredicate<Rhs>: Functor<Lhs>,
{
    type Output = And<Lhs, Rhs>;
}

// or op

/// A [Predicate] that joins input and `Rhs` with [Boolean] type.
pub struct OrPredicate<Rhs>
where
    Rhs: Boolean,
{
    _phantom: PhantomData<Rhs>,
}

pub type Or<Lhs, Rhs> = ApplyFunctor<OrPredicate<Rhs>, Lhs>;

impl<Lhs, Rhs> Predicate<Lhs> for OrPredicate<Rhs>
where
    Lhs: Boolean,
    Rhs: Boolean,
    OrPredicate<Rhs>: Functor<Lhs>,
    Or<Lhs, Rhs>: Boolean,
{
}

impl Functor<True> for OrPredicate<True> {
    type Output = True;
}

impl Functor<False> for OrPredicate<True> {
    type Output = True;
}

impl Functor<True> for OrPredicate<False> {
    type Output = True;
}

impl Functor<False> for OrPredicate<False> {
    type Output = False;
}

/// A [Predicate] that joins input pair `(Lhs, Rhs)` with [Boolean] type.
pub struct OrComposePredicate;

pub type OrCompose<Lhs, Rhs> = ApplyFunctor<OrComposePredicate, (Lhs, Rhs)>;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for OrComposePredicate
where
    Lhs: Boolean,
    Rhs: Boolean,
    OrPredicate<Rhs>: Functor<Lhs>,
{
    type Output = Or<Lhs, Rhs>;
}

// not op

/// A [Predicate] that inverts [Boolean] types.
pub struct NotPredicate;

pub type Not<Input> = ApplyFunctor<NotPredicate, Input>;

impl<Input> Predicate<Input> for NotPredicate
where
    Input: Boolean,
    NotPredicate: Functor<Input>,
    Not<Input>: Boolean,
{
}

impl Functor<True> for NotPredicate {
    type Output = False;
}

impl Functor<False> for NotPredicate {
    type Output = True;
}

// xor op

/// A [Predicate] that computes exclusive-or on input and `Rhs` with [Boolean] types.
pub struct XorPredicate<Rhs>
where
    Rhs: Boolean,
{
    _phantom: PhantomData<Rhs>,
}

pub type Xor<Lhs, Rhs> = ApplyFunctor<XorPredicate<Rhs>, Lhs>;

impl<Lhs, Rhs> Predicate<Lhs> for XorPredicate<Rhs>
where
    Lhs: Boolean,
    Rhs: Boolean,
    XorPredicate<Rhs>: Functor<Lhs>,
    Xor<Lhs, Rhs>: Boolean,
{
}

impl Functor<True> for XorPredicate<True> {
    type Output = False;
}

impl Functor<False> for XorPredicate<True> {
    type Output = True;
}

impl Functor<True> for XorPredicate<False> {
    type Output = True;
}

impl Functor<False> for XorPredicate<False> {
    type Output = False;
}

/// A [Predicate] that computes exclusive-or on input pair `(Lhs, Rhs)`.
pub struct XorComposePredicate;

pub type XorCompose<Lhs, Rhs> = ApplyFunctor<XorComposePredicate, (Lhs, Rhs)>;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for XorComposePredicate
where
    Lhs: Boolean,
    Rhs: Boolean,
    XorPredicate<Rhs>: Functor<Lhs>,
{
    type Output = Xor<Lhs, Rhs>;
}

// iff op

/// A [Predicate] that returns of both input and `Rhs` have same [Boolean] values.
pub struct IffPredicate<Rhs>
where
    Rhs: Boolean,
{
    _phantom: PhantomData<Rhs>,
}

pub type Iff<Lhs, Rhs> = ApplyFunctor<IffPredicate<Rhs>, Lhs>;

impl<Lhs, Rhs> Predicate<Lhs> for IffPredicate<Rhs>
where
    Lhs: Boolean,
    Rhs: Boolean,
    IffPredicate<Rhs>: Functor<Lhs>,
    Iff<Lhs, Rhs>: Boolean,
{
}

impl Functor<True> for IffPredicate<True> {
    type Output = True;
}

impl Functor<False> for IffPredicate<True> {
    type Output = False;
}

impl Functor<True> for IffPredicate<False> {
    type Output = False;
}

impl Functor<False> for IffPredicate<False> {
    type Output = True;
}

/// A [Predicate] that returns if pair of input types `(Lhs, Rhs)` have same [Boolean] value.
pub struct IffComposePredicate;

pub type IffCompose<Lhs, Rhs> = ApplyFunctor<IffComposePredicate, (Lhs, Rhs)>;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for IffComposePredicate
where
    Lhs: Boolean,
    Rhs: Boolean,
    IffPredicate<Rhs>: Functor<Lhs>,
{
    type Output = Iff<Lhs, Rhs>;
}

// imply op

/// A [Predicate] that returns if input implies to `Rhs`.
pub struct ImplyPredicate<Rhs>
where
    Rhs: Boolean,
{
    _phantom: PhantomData<Rhs>,
}

pub type Imply<Lhs, Rhs> = ApplyFunctor<ImplyPredicate<Rhs>, Lhs>;

impl<Lhs, Rhs> Predicate<Lhs> for ImplyPredicate<Rhs>
where
    Lhs: Boolean,
    Rhs: Boolean,
    ImplyPredicate<Rhs>: Functor<Lhs>,
    Imply<Lhs, Rhs>: Boolean,
{
}

impl Functor<True> for ImplyPredicate<True> {
    type Output = True;
}

impl Functor<False> for ImplyPredicate<True> {
    type Output = True;
}

impl Functor<True> for ImplyPredicate<False> {
    type Output = False;
}

impl Functor<False> for ImplyPredicate<False> {
    type Output = True;
}

/// A [Predicate] that computes if `Lhs` implies `Rhs` for input pair `(Lhs, Rhs)`.
pub struct ImplyComposePredicate;

pub type ImplyCompose<Lhs, Rhs> = ApplyFunctor<ImplyComposePredicate, (Lhs, Rhs)>;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for ImplyComposePredicate
where
    Lhs: Boolean,
    Rhs: Boolean,
    ImplyPredicate<Rhs>: Functor<Lhs>,
{
    type Output = Imply<Lhs, Rhs>;
}

// not imply op

/// A [Predicate] that returns if input is true while `Rhs` is false.
pub struct NotImplyPredicate<Rhs>
where
    Rhs: Boolean,
{
    _phantom: PhantomData<Rhs>,
}

pub type NotImply<Lhs, Rhs> = ApplyFunctor<NotImplyPredicate<Rhs>, Lhs>;

impl<Lhs, Rhs> Predicate<Lhs> for NotImplyPredicate<Rhs>
where
    Lhs: Boolean,
    Rhs: Boolean,
    NotImplyPredicate<Rhs>: Functor<Lhs>,
    NotImply<Lhs, Rhs>: Boolean,
{
}

impl Functor<True> for NotImplyPredicate<True> {
    type Output = False;
}

impl Functor<False> for NotImplyPredicate<True> {
    type Output = False;
}

impl Functor<True> for NotImplyPredicate<False> {
    type Output = True;
}

impl Functor<False> for NotImplyPredicate<False> {
    type Output = False;
}

/// A [Predicate] that computes if `Lhs` is true while `Rhs` is false for input pair `(Lhs, Rhs)`.
pub struct NotImplyComposePredicate;

pub type NotImplyCompose<Lhs, Rhs> = ApplyFunctor<NotImplyComposePredicate, (Lhs, Rhs)>;

impl<Lhs, Rhs> Functor<(Lhs, Rhs)> for NotImplyComposePredicate
where
    Lhs: Boolean,
    Rhs: Boolean,
    NotImplyPredicate<Rhs>: Functor<Lhs>,
{
    type Output = NotImply<Lhs, Rhs>;
}
