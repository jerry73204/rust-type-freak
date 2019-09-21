use typenum::{B0, B1};

// boolean type def

/// A trait that provides boolean constant value.
pub trait Boolean {
    const BOOL: bool;
}

/// An alias of [typenum::B0].
pub type False = B0;

impl Boolean for False {
    const BOOL: bool = false;
}

/// An alias of [typenum::B1].
pub type True = B1;

impl Boolean for True {
    const BOOL: bool = true;
}

// and op

/// A trait operator that joins two [Boolean] types.
pub trait And<Rhs>
where
    Self: Boolean,
    Self::Out: Boolean,
{
    type Out;
}

pub type AndOut<Lhs, Rhs> = <Lhs as And<Rhs>>::Out;

impl And<True> for True {
    type Out = True;
}

impl And<True> for False {
    type Out = False;
}

impl And<False> for True {
    type Out = False;
}

impl And<False> for False {
    type Out = False;
}

// or op

/// A trait operator that meets two [Boolean] types.
pub trait Or<Rhs>
where
    Self: Boolean,
    Self::Out: Boolean,
{
    type Out;
}

pub type OrOut<Lhs, Rhs> = <Lhs as Or<Rhs>>::Out;

impl Or<True> for True {
    type Out = True;
}

impl Or<True> for False {
    type Out = True;
}

impl Or<False> for True {
    type Out = True;
}

impl Or<False> for False {
    type Out = False;
}

// not op

/// A trait operator that inverts [Boolean] types.
pub trait Not
where
    Self: Boolean,
    Self::Out: Boolean,
{
    type Out;
}

pub type NotOut<In> = <In as Not>::Out;

impl Not for True {
    type Out = False;
}

impl Not for False {
    type Out = True;
}

// xor op

/// A trait operator that takes exclusive-or of two [Boolean] types.
pub trait Xor<Rhs>
where
    Self: Boolean,
    Self::Out: Boolean,
{
    type Out;
}

pub type XorOut<Lhs, Rhs> = <Lhs as Xor<Rhs>>::Out;

impl Xor<True> for True {
    type Out = False;
}

impl Xor<True> for False {
    type Out = True;
}

impl Xor<False> for True {
    type Out = True;
}

impl Xor<False> for False {
    type Out = False;
}

// iff op

/// A trait operator that checks if two [Boolean] types are equal.
pub trait Iff<Rhs>
where
    Self: Boolean,
    Self::Out: Boolean,
{
    type Out;
}

pub type IffOut<Lhs, Rhs> = <Lhs as Iff<Rhs>>::Out;

impl Iff<True> for True {
    type Out = True;
}

impl Iff<True> for False {
    type Out = False;
}

impl Iff<False> for True {
    type Out = False;
}

impl Iff<False> for False {
    type Out = True;
}
