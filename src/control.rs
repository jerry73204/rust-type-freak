use crate::{boolean::Boolean, tuple::SecondOfOutput};
use typenum::{
    Eq, False, Gr, GrEq, IsEqual, IsGreater, IsGreaterOrEqual, IsLess, IsLessOrEqual, Le, LeEq,
    True,
};

// if

/// A type operator that checks if type can be constructed.
pub trait If<Cond> {
    type Output;
}

pub type IfOutput<Output, Cond> = <Output as If<Cond>>::Output;

impl<Cond, Output> If<Cond> for Output {
    type Output = SecondOfOutput<(Cond, Output)>;
}

// if type equivalence

/// A type operator that checks you both types are equivalent.
pub trait IfSame<Lhs, Rhs> {
    type Output;
}

pub type IfSameOutput<Output, Lhs, Rhs> = <Output as IfSame<Lhs, Rhs>>::Output;

impl<Same, Output> IfSame<Same, Same> for Output {
    type Output = Output;
}

// if predicate

/// A type operator that checks if condition is [True](crate::boolean::True).
pub trait IfPredicate<Cond>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfPredicateOutput<Output, Cond> = <Output as IfPredicate<Cond>>::Output;

impl<Output> IfPredicate<True> for Output {
    type Output = Output;
}

// if-else predicate

/// A type operator that returns output depending [Boolean](crate::boolean::Boolean) condition.
pub trait IfElsePredicate<Cond>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfElsePredicateOutput<TrueOutput, FalseOutput, Cond> =
    <(TrueOutput, FalseOutput) as IfElsePredicate<Cond>>::Output;

impl<TrueOutput, FalseOutput> IfElsePredicate<True> for (TrueOutput, FalseOutput) {
    type Output = TrueOutput;
}

impl<TrueOutput, FalseOutput> IfElsePredicate<False> for (TrueOutput, FalseOutput) {
    type Output = FalseOutput;
}

// if not predicate

/// A type operator that checks if condition is [False](crate::boolean::False).
pub trait IfNotPredicate<Cond>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfNotPredicateOutput<Output, Cond> = <Output as IfPredicate<Cond>>::Output;

impl<Output> IfNotPredicate<False> for Output {
    type Output = Output;
}

// if less than

/// A type operator that checks if left-hand-site is less than right-hand-side.
pub trait IfLess<Lhs, Rhs> {
    type Output;
}

pub type IfLessOutput<Output, Lhs, Rhs> = <Output as IfLess<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfLess<Lhs, Rhs> for Output
where
    Lhs: IsLess<Rhs>,
    Output: IfPredicate<Le<Lhs, Rhs>>,
    Le<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Le<Lhs, Rhs>>;
}

// if less than or equal

/// A type operator that checks if left-hand-site is less than or equals to right-hand-side.
pub trait IfLessOrEqual<Lhs, Rhs> {
    type Output;
}

pub type IfLessOrEqualOutput<Output, Lhs, Rhs> = <Output as IfLessOrEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfLessOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsLessOrEqual<Rhs>,
    Output: IfPredicate<LeEq<Lhs, Rhs>>,
    LeEq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, LeEq<Lhs, Rhs>>;
}

// if greater than

/// A type operator that checks if left-hand-site is greater than right-hand-side.
pub trait IfGreater<Lhs, Rhs> {
    type Output;
}

pub type IfGreaterOutput<Output, Lhs, Rhs> = <Output as IfGreater<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfGreater<Lhs, Rhs> for Output
where
    Lhs: IsGreater<Rhs>,
    Output: IfPredicate<Gr<Lhs, Rhs>>,
    Gr<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Gr<Lhs, Rhs>>;
}

// if greater than or equal

/// A type operator that checks if left-hand-site is greater than or equals to right-hand-side.
pub trait IfGreaterOrEqual<Lhs, Rhs> {
    type Output;
}

pub type IfGreaterOrEqualOutput<Output, Lhs, Rhs> = <Output as IfGreaterOrEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfGreaterOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsGreaterOrEqual<Rhs>,
    Output: IfPredicate<GrEq<Lhs, Rhs>>,
    GrEq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, GrEq<Lhs, Rhs>>;
}

// if equal

/// A type operator that checks if left-hand-site equals to right-hand-side.
pub trait IfEqual<Lhs, Rhs> {
    type Output;
}

pub type IfEqualOutput<Output, Lhs, Rhs> = <Output as IfEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfEqual<Lhs, Rhs> for Output
where
    Lhs: IsEqual<Rhs>,
    Output: IfPredicate<Eq<Lhs, Rhs>>,
    Eq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Eq<Lhs, Rhs>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{consts::*, Le, Unsigned};

    type Assert1 = IfOutput<U3, ()>;
    type Assert2 = IfSameOutput<(), u8, u8>;
    type Assert3 = IfPredicateOutput<(), Le<U3, U4>>;
    type Assert4 = IfElsePredicateOutput<True, False, Le<U3, U4>>;

    type Assert5 = IfLessOutput<(), U6, U9>;

    type Assert6 = IfLessOrEqualOutput<(), U6, U6>;
    type Assert7 = IfLessOrEqualOutput<(), U6, U7>;

    type Assert8 = IfGreaterOutput<(), U7, U4>;

    type Assert9 = IfGreaterOrEqualOutput<(), U7, U4>;
    type Assert10 = IfGreaterOrEqualOutput<(), U7, U7>;

    type Assert11 = IfEqualOutput<(), Z0, Z0>;

    #[test]
    fn control_test() {
        // if constructed
        assert_eq!(3, Assert1::USIZE);

        // if type equivalence
        let _: Assert2 = ();

        // if predicate
        let _: Assert3 = ();

        // if else predicate
        assert!(Assert4::BOOL);

        // if less than
        let _: Assert5 = ();

        // if less than or equal
        let _: Assert6 = ();
        let _: Assert7 = ();

        // if greater than
        let _: Assert8 = ();

        // if greater than or equal
        let _: Assert9 = ();
        let _: Assert10 = ();

        // if equal
        let _: Assert11 = ();
    }
}
