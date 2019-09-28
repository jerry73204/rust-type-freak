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
//! type R1 = NotOutput<L>;     // False
//! type R2 = AndOutput<L, R>;  // False
//! type R3 = OrOutput<L, R>;   // True
//! type R4 = XorOutput<L, R>;  // True
//! type R5 = IffOutput<L, R>;  // False
//!
//! fn get_value() -> bool {
//!     R1::BOOL  // Get constant value
//! }
//! ```

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

// assert true

/// An type operator that outputs `()` if input is [True].
pub trait AssertTrue
where
    Self: Boolean,
{
    type Output;
}

pub type AssertTrueOutput<Input> = <Input as AssertTrue>::Output;

impl AssertTrue for True {
    type Output = ();
}

// assert false

/// An type operator that outputs `()` if input is [False].
pub trait AssertFalse
where
    Self: Boolean,
{
    type Output;
}

pub type AssertFalseOutput<Input> = <Input as AssertFalse>::Output;

impl AssertFalse for False {
    type Output = ();
}

// and op

/// A type operator that joins two [Boolean] types.
pub trait And<Rhs>
where
    Self: Boolean,
    Self::Output: Boolean,
{
    type Output;
}

pub type AndOutput<Lhs, Rhs> = <Lhs as And<Rhs>>::Output;

impl And<True> for True {
    type Output = True;
}

impl And<True> for False {
    type Output = False;
}

impl And<False> for True {
    type Output = False;
}

impl And<False> for False {
    type Output = False;
}

// or op

/// A type operator that meets two [Boolean] types.
pub trait Or<Rhs>
where
    Self: Boolean,
    Self::Output: Boolean,
{
    type Output;
}

pub type OrOutput<Lhs, Rhs> = <Lhs as Or<Rhs>>::Output;

impl Or<True> for True {
    type Output = True;
}

impl Or<True> for False {
    type Output = True;
}

impl Or<False> for True {
    type Output = True;
}

impl Or<False> for False {
    type Output = False;
}

// not op

/// A type operator that inverts [Boolean] types.
pub trait Not
where
    Self: Boolean,
    Self::Output: Boolean,
{
    type Output;
}

pub type NotOutput<In> = <In as Not>::Output;

impl Not for True {
    type Output = False;
}

impl Not for False {
    type Output = True;
}

// xor op

/// A type operator that takes exclusive-or of two [Boolean] types.
pub trait Xor<Rhs>
where
    Self: Boolean,
    Self::Output: Boolean,
{
    type Output;
}

pub type XorOutput<Lhs, Rhs> = <Lhs as Xor<Rhs>>::Output;

impl Xor<True> for True {
    type Output = False;
}

impl Xor<True> for False {
    type Output = True;
}

impl Xor<False> for True {
    type Output = True;
}

impl Xor<False> for False {
    type Output = False;
}

// iff op

/// A type operator that checks if two [Boolean] types are equal.
pub trait Iff<Rhs>
where
    Self: Boolean,
    Self::Output: Boolean,
{
    type Output;
}

pub type IffOutput<Lhs, Rhs> = <Lhs as Iff<Rhs>>::Output;

impl Iff<True> for True {
    type Output = True;
}

impl Iff<True> for False {
    type Output = False;
}

impl Iff<False> for True {
    type Output = False;
}

impl Iff<False> for False {
    type Output = True;
}

// imply op

/// A type operator that checks if Lhs implies Rhs.
pub trait Imply<Rhs>
where
    Self: Boolean,
    Self::Output: Boolean,
{
    type Output;
}

pub type ImplyOutput<Lhs, Rhs> = <Lhs as Imply<Rhs>>::Output;

impl<Lhs, Rhs> Imply<Rhs> for Lhs
where
    Lhs: Boolean + Not,
    Rhs: Boolean + Or<NotOutput<Lhs>>,
{
    type Output = OrOutput<Rhs, NotOutput<Lhs>>;
}

// not imply op

/// A type operator that checks if Lhs does not imply Rhs.
pub trait NotImply<Rhs>
where
    Self: Boolean,
    Self::Output: Boolean,
{
    type Output;
}

pub type NotImplyOutput<Lhs, Rhs> = <Lhs as NotImply<Rhs>>::Output;

impl<Lhs, Rhs> NotImply<Rhs> for Lhs
where
    Lhs: Boolean + Imply<Rhs>,
    Rhs: Boolean,
    ImplyOutput<Lhs, Rhs>: Not,
{
    type Output = NotOutput<ImplyOutput<Lhs, Rhs>>;
}
