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
