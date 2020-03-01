//! Compile-time guards and static assertions.
//!
//! ## Overview
//! Most type operators in this module copies the input type to `Self::Output`
//! when certain conditions are met. We have these categories of operators:
//!
//! - [If<Output, Type>](crate::control::IfOutput):
//!     Asserts `Type` can be constructed.
//! - [IfPredicate<Output, Predicate>](crate::control::IfPredicate):
//!     Asserts `Predicate` derives to [True](typenum::True).
//! - [IfSame<Output, Lhs, Rhs>](crate::control::IfSame):
//!     Asserts `Lhs` and `Rhs` are of the same type.
//! - [IfLess](crate::control::IfLess), [IfLessOrEqual](crate::control::IfLessOrEqual),
//!     [IfGreater](crate::control::IfGreater), [IfGreaterOrEqual](crate::control::IfGreaterOrEqual),
//!     [IfEqual](crate::control::IfEqual):
//!     Asserts two [typenum] numbers follows the order.
//!
//! By convention, [IfSameOutput<Output, Lhs, Rhs>](crate::control::IfSameOutput) is type alias of
//! `<Output as IfSame<Lhs, Rhs>>::Output` trait cast, and others follow.
//! Only [IfOutput<Output, Type>](crate::control::IfOutput) has no corresponding trait.
//!
//! ## Static assertions
//! We can make use of `If*Output` aliases to build compile time assertions.
//! For example, [IfLessOutput](crate::control::IfLessOutput) asserts LHS
//! is less than RHS.
//!
//! ```ignore
//! use typenum::consts::*;
//! use type_freak::control::IfLessOutput;
//!
//! type Out1 = IfLessOutput<usize, U3, U5>;  // U3 < U5 is true, thus Out1 ~= usize
//! type Out2 = IfLessOutput<usize, U5, U3>;  // U5 < U5 is false
//!
//! fn assert() {
//!    let _: Out1 = 0;  // Goes fine here.
//!    let _: Out2 = 0;  // Compile error!!!
//! }
//!  ```
//!
//! ## Compile-time guards
//! By placing `If*` trait bounds in `where` block. we can build compile-time
//! guarded functions. For example, we add `IfSame` trait bound to assert two function
//! generic parameters have identical types.
//! The same trick applies to guarded structs, traits and impl blocks.
//!
//! ```ignore
//! use type_freak::control::IfSame;
//!
//! fn guarded_function<Lhs, Rhs>() -> String
//! where
//!     Lhs: IfSame<Lhs, Rhs>
//! {
//!     "Yeeeeeee!".to_owned()
//! }
//!
//! fn comile_me() {
//!     let _ = guarded_function::<String, String>();  // fine
//!     let _ = guarded_function::<String, u8>();      // Compile error!!!
//! }
//! ```

// use crate::{boolean::Boolean,};
use typenum::{
    Bit, Eq, False, Gr, GrEq, IsEqual, IsGreater, IsGreaterOrEqual, IsLess, IsLessOrEqual,
    IsNotEqual, Le, LeEq, NInt, NonZero, NotEq, PInt, True, UInt, UTerm, Unsigned, B0, B1, U0, Z0,
};

pub mod ops {
    use super::*;

    // assert type equivalence

    /// Returns input type if both `Lhs` and `Rhs` are equivalent types.
    pub trait AssertSame<Lhs, Rhs, Output> {
        type Output;
    }

    impl<Same, Output> AssertSame<Same, Same, Output> for () {
        type Output = Output;
    }

    // assert boolean predicate

    /// Returns input type if `Cond` evaluates to [True].
    pub trait AssertPredicate<Cond, Output>
    where
        Cond: Bit,
    {
        type Output;
    }

    impl<Output> AssertPredicate<True, Output> for () {
        type Output = Output;
    }

    // assert predicate is false

    /// Returns input type if `Cond` evaluates to [False].
    pub trait AssertNotPredicate<Cond, Output>
    where
        Cond: Bit,
    {
        type Output;
    }

    impl<Output> AssertNotPredicate<False, Output> for () {
        type Output = Output;
    }

    // if-else predicate

    /// Returns input type if `Cond` evaluates to [True], otherwise returns `Else`.
    pub trait IfElsePredicate<Cond, TrueOutput, FalseOutput>
    where
        Cond: Bit,
    {
        type Output;
    }

    impl<TrueOutput, FalseOutput> IfElsePredicate<True, TrueOutput, FalseOutput> for () {
        type Output = TrueOutput;
    }

    impl<TrueOutput, FalseOutput> IfElsePredicate<False, TrueOutput, FalseOutput> for () {
        type Output = FalseOutput;
    }

    // if-else not predicate

    /// Returns input type if `Cond` evaluates to [True], otherwise returns `Else`.
    pub trait IfElseNotPredicate<Cond, TrueOutput, FalseOutput>
    where
        Cond: Bit,
    {
        type Output;
    }

    impl<TrueOutput, FalseOutput> IfElseNotPredicate<True, TrueOutput, FalseOutput> for () {
        type Output = FalseOutput;
    }

    impl<TrueOutput, FalseOutput> IfElseNotPredicate<False, TrueOutput, FalseOutput> for () {
        type Output = TrueOutput;
    }

    // assert less than

    /// Returns input type if `Lhs` is less than `Rhs`.
    pub trait AssertLess<Lhs, Rhs, Output> {
        type Output;
    }

    impl<Lhs, Rhs, Output> AssertLess<Lhs, Rhs, Output> for ()
    where
        (): AssertPredicate<Le<Lhs, Rhs>, Output>,
        Lhs: IsLess<Rhs>,
        Le<Lhs, Rhs>: Bit,
    {
        type Output = Output;
    }

    // if-else less than

    /// Returns input type if `Lhs` is less than `Rhs`, otherwise returns `Else`.
    pub trait IfElseLess<Lhs, Rhs, TrueOutput, FalseOutput> {
        type Output;
    }

    impl<Lhs, Rhs, TrueOutput, FalseOutput> IfElseLess<Lhs, Rhs, TrueOutput, FalseOutput> for ()
    where
        Lhs: IsLess<Rhs>,
        (): IfElsePredicate<Le<Lhs, Rhs>, TrueOutput, FalseOutput>,
        Le<Lhs, Rhs>: Bit,
    {
        type Output = op_aliases::IfElsePredicate<Le<Lhs, Rhs>, TrueOutput, FalseOutput>;
    }

    // assert less than or equal

    /// Returns input type if `Lhs` is less than or equals to `Rhs`.
    pub trait AssertLessOrEqual<Lhs, Rhs, Output> {
        type Output;
    }

    impl<Lhs, Rhs, Output> AssertLessOrEqual<Lhs, Rhs, Output> for ()
    where
        (): AssertPredicate<LeEq<Lhs, Rhs>, Output>,
        Lhs: IsLessOrEqual<Rhs>,
        LeEq<Lhs, Rhs>: Bit,
    {
        type Output = Output;
    }

    // if-else less or equal

    /// Returns input type if `Lhs` is less than or equals to `Rhs`, otherwise returns `Else`.
    pub trait IfElseLessOrEqual<Lhs, Rhs, TrueOutput, FalseOutput> {
        type Output;
    }

    impl<Lhs, Rhs, TrueOutput, FalseOutput> IfElseLessOrEqual<Lhs, Rhs, TrueOutput, FalseOutput> for ()
    where
        (): IfElsePredicate<LeEq<Lhs, Rhs>, TrueOutput, FalseOutput>,
        Lhs: IsLessOrEqual<Rhs>,
        LeEq<Lhs, Rhs>: Bit,
    {
        type Output = op_aliases::IfElsePredicate<LeEq<Lhs, Rhs>, TrueOutput, FalseOutput>;
    }

    // assert greater than

    /// Returns input type if `Lhs` is greater than `Rhs`.
    pub trait AssertGreater<Lhs, Rhs, Output> {
        type Output;
    }

    impl<Lhs, Rhs, Output> AssertGreater<Lhs, Rhs, Output> for ()
    where
        (): AssertPredicate<Gr<Lhs, Rhs>, Output>,
        Lhs: IsGreater<Rhs>,
        Gr<Lhs, Rhs>: Bit,
    {
        type Output = Output;
    }

    // if-else greater than

    /// Returns input type if `Lhs` is greater than `Rhs`, otherwise returns `Else`.
    pub trait IfElseGreater<Lhs, Rhs, TrueOutput, FalseOutput> {
        type Output;
    }

    impl<Lhs, Rhs, TrueOutput, FalseOutput> IfElseGreater<Lhs, Rhs, TrueOutput, FalseOutput> for ()
    where
        (): IfElsePredicate<Gr<Lhs, Rhs>, TrueOutput, FalseOutput>,
        Lhs: IsGreater<Rhs>,
        Gr<Lhs, Rhs>: Bit,
    {
        type Output = op_aliases::IfElsePredicate<Gr<Lhs, Rhs>, TrueOutput, FalseOutput>;
    }

    // assert greater than or equal

    /// Returns input type if `Lhs` is greater than or equals to `Rhs`.
    pub trait AssertGreaterOrEqual<Lhs, Rhs, Output> {
        type Output;
    }

    impl<Lhs, Rhs, Output> AssertGreaterOrEqual<Lhs, Rhs, Output> for ()
    where
        (): AssertPredicate<GrEq<Lhs, Rhs>, Output>,
        Lhs: IsGreaterOrEqual<Rhs>,
        GrEq<Lhs, Rhs>: Bit,
    {
        type Output = Output;
    }

    // if-else greater or equal

    /// Returns input type if `Lhs` is greater than or equals to `Rhs`, otherwise returns `Else`.
    pub trait IfElseGreaterOrEqual<Lhs, Rhs, TrueOutput, FalseOutput> {
        type Output;
    }

    impl<Lhs, Rhs, TrueOutput, FalseOutput> IfElseGreaterOrEqual<Lhs, Rhs, TrueOutput, FalseOutput>
        for ()
    where
        (): IfElsePredicate<GrEq<Lhs, Rhs>, TrueOutput, FalseOutput>,
        Lhs: IsGreaterOrEqual<Rhs>,
        GrEq<Lhs, Rhs>: Bit,
    {
        type Output = op_aliases::IfElsePredicate<GrEq<Lhs, Rhs>, TrueOutput, FalseOutput>;
    }

    // assert equal

    /// Returns input type if `Lhs` equals to `Rhs`.
    pub trait AssertEqual<Lhs, Rhs, Output> {
        type Output;
    }

    impl<Lhs, Rhs, Output> AssertEqual<Lhs, Rhs, Output> for ()
    where
        (): AssertPredicate<Eq<Lhs, Rhs>, Output>,
        Lhs: IsEqual<Rhs>,
        Eq<Lhs, Rhs>: Bit,
    {
        type Output = Output;
    }

    // if else equal

    /// Returns output if left-hand-site equals to right-hand-side, otherwise returns `Else`.
    pub trait IfElseEqual<Lhs, Rhs, TrueOutput, FalseOutput> {
        type Output;
    }

    impl<Lhs, Rhs, TrueOutput, FalseOutput> IfElseEqual<Lhs, Rhs, TrueOutput, FalseOutput> for ()
    where
        (): IfElsePredicate<Eq<Lhs, Rhs>, TrueOutput, FalseOutput>,
        Lhs: IsEqual<Rhs>,
        Eq<Lhs, Rhs>: Bit,
    {
        type Output = op_aliases::IfElsePredicate<Eq<Lhs, Rhs>, TrueOutput, FalseOutput>;
    }

    // assert not equal

    /// Returns input type if `Lhs` equals to `Rhs`.
    pub trait AssertNotEqual<Lhs, Rhs, Output> {
        type Output;
    }

    impl<Lhs, Rhs, Output> AssertNotEqual<Lhs, Rhs, Output> for ()
    where
        (): AssertPredicate<NotEq<Lhs, Rhs>, Output>,
        Lhs: IsNotEqual<Rhs>,
        NotEq<Lhs, Rhs>: Bit,
    {
        type Output = Output;
    }

    // if else not equal

    /// Returns output if left-hand-site equals to right-hand-side, otherwise returns `Else`.
    pub trait IfElseNotEqual<Lhs, Rhs, TrueOutput, FalseOutput> {
        type Output;
    }

    impl<Lhs, Rhs, TrueOutput, FalseOutput> IfElseNotEqual<Lhs, Rhs, TrueOutput, FalseOutput> for ()
    where
        (): IfElsePredicate<NotEq<Lhs, Rhs>, TrueOutput, FalseOutput>,
        Lhs: IsNotEqual<Rhs>,
        NotEq<Lhs, Rhs>: Bit,
    {
        type Output = op_aliases::IfElsePredicate<NotEq<Lhs, Rhs>, TrueOutput, FalseOutput>;
    }

    // assert zero

    /// A type operator that checks if a [typenum] value is either
    /// [B0](typenum::B0), [Z0](typenum::Z0) or [U0](typenum::U0).
    pub trait AssertZero<Value, Output> {
        type Output;
    }

    impl<Output> AssertZero<B0, Output> for () {
        type Output = Output;
    }

    impl<Output> AssertZero<Z0, Output> for () {
        type Output = Output;
    }

    impl<Output> AssertZero<U0, Output> for () {
        type Output = Output;
    }

    // if-else zero

    pub trait IfElseZero<Value, TrueOutput, FalseOutput> {
        type Output;
    }

    impl<TrueOutput, FalseOutput> IfElseZero<B0, TrueOutput, FalseOutput> for () {
        type Output = TrueOutput;
    }

    impl<TrueOutput, FalseOutput> IfElseZero<B1, TrueOutput, FalseOutput> for () {
        type Output = FalseOutput;
    }

    impl<TrueOutput, FalseOutput> IfElseZero<Z0, TrueOutput, FalseOutput> for () {
        type Output = TrueOutput;
    }

    impl<TrueOutput, FalseOutput, U> IfElseZero<PInt<U>, TrueOutput, FalseOutput> for ()
    where
        U: Unsigned + NonZero,
    {
        type Output = FalseOutput;
    }

    impl<TrueOutput, FalseOutput, U> IfElseZero<NInt<U>, TrueOutput, FalseOutput> for ()
    where
        U: Unsigned + NonZero,
    {
        type Output = FalseOutput;
    }

    impl<TrueOutput, FalseOutput> IfElseZero<UTerm, TrueOutput, FalseOutput> for () {
        type Output = TrueOutput;
    }

    impl<TrueOutput, FalseOutput, U, B> IfElseZero<UInt<U, B>, TrueOutput, FalseOutput> for ()
    where
        U: Unsigned,
        B: Bit,
    {
        type Output = FalseOutput;
    }

    // assert non-zero

    /// A type operator that checks if a [typenum] value implements
    /// [NonZero](typenum::NonZero) trait.
    pub trait AssertNonZero<Value, Output>
    where
        Value: NonZero,
    {
        type Output;
    }

    impl<Value, Output> AssertNonZero<Value, Output> for ()
    where
        Value: NonZero,
    {
        type Output = Output;
    }

    // if-else non-zero

    pub trait IfElseNonZero<Value, TrueOutput, FalseOutput> {
        type Output;
    }

    impl<Value, TrueOutput, FalseOutput> IfElseNonZero<Value, TrueOutput, FalseOutput> for ()
    where
        (): IfElseZero<Value, FalseOutput, TrueOutput>,
    {
        type Output = op_aliases::IfElseZero<Value, FalseOutput, TrueOutput>;
    }
}

pub mod op_aliases {
    use super::*;

    pub type AssertSame<Lhs, Rhs, Output> = <() as ops::AssertSame<Lhs, Rhs, Output>>::Output;
    pub type AssertPredicate<Cond, Output> = <() as ops::AssertPredicate<Cond, Output>>::Output;
    pub type AssertNotPredicate<Cond, Output> =
        <() as ops::AssertNotPredicate<Cond, Output>>::Output;
    pub type AssertEqual<Lhs, Rhs, Output> = <() as ops::AssertEqual<Lhs, Rhs, Output>>::Output;
    pub type AssertNotEqual<Lhs, Rhs, Output> =
        <() as ops::AssertNotEqual<Lhs, Rhs, Output>>::Output;
    pub type AssertLess<Lhs, Rhs, Output> = <() as ops::AssertLess<Lhs, Rhs, Output>>::Output;
    pub type AssertLessOrEqual<Lhs, Rhs, Output> =
        <() as ops::AssertLessOrEqual<Lhs, Rhs, Output>>::Output;
    pub type AssertGreater<Lhs, Rhs, Output> = <() as ops::AssertGreater<Lhs, Rhs, Output>>::Output;
    pub type AssertGreaterOrEqual<Lhs, Rhs, Output> =
        <() as ops::AssertGreaterOrEqual<Lhs, Rhs, Output>>::Output;
    pub type AssertZero<Value, Output> = <() as ops::AssertZero<Value, Output>>::Output;
    pub type AssertNonZero<Value, Output> = <() as ops::AssertNonZero<Value, Output>>::Output;

    pub type IfElsePredicate<Cond, TrueOutput, FalseOutput> =
        <() as ops::IfElsePredicate<Cond, TrueOutput, FalseOutput>>::Output;
    pub type IfElseNotPredicate<Cond, TrueOutput, FalseOutput> =
        <() as ops::IfElseNotPredicate<Cond, TrueOutput, FalseOutput>>::Output;
    pub type IfElseEqual<Lhs, Rhs, TrueOutput, FalseOutput> =
        <() as ops::IfElseEqual<Lhs, Rhs, TrueOutput, FalseOutput>>::Output;
    pub type IfElseNotEqual<Lhs, Rhs, TrueOutput, FalseOutput> =
        <() as ops::IfElseNotEqual<Lhs, Rhs, TrueOutput, FalseOutput>>::Output;
    pub type IfElseLess<Lhs, Rhs, TrueOutput, FalseOutput> =
        <() as ops::IfElseLess<Lhs, Rhs, TrueOutput, FalseOutput>>::Output;
    pub type IfElseLessOrEqual<Lhs, Rhs, TrueOutput, FalseOutput> =
        <() as ops::IfElseLessOrEqual<Lhs, Rhs, TrueOutput, FalseOutput>>::Output;
    pub type IfElseGreater<Lhs, Rhs, TrueOutput, FalseOutput> =
        <() as ops::IfElseGreater<Lhs, Rhs, TrueOutput, FalseOutput>>::Output;
    pub type IfElseGreaterOrEqual<Lhs, Rhs, TrueOutput, FalseOutput> =
        <() as ops::IfElseGreaterOrEqual<Lhs, Rhs, TrueOutput, FalseOutput>>::Output;
    pub type IfElseZero<Value, TrueOutput, FalseOutput> =
        <() as ops::IfElseZero<Value, TrueOutput, FalseOutput>>::Output;
    pub type IfElseNonZero<Value, TrueOutput, FalseOutput> =
        <() as ops::IfElseNonZero<Value, TrueOutput, FalseOutput>>::Output;

}

#[cfg(test)]
mod tests {
    use super::op_aliases::*;
    use typenum::consts::*;

    type Assert1 = AssertSame<((), ()), ((), ()), ()>;
    type Assert2 = AssertPredicate<True, ()>;
    type Assert3 = AssertNotPredicate<False, ()>;
    type Assert4 = AssertEqual<U1, U1, ()>;
    type Assert5 = AssertLess<U1, U2, ()>;
    type Assert6 = AssertLessOrEqual<U1, U2, ()>;
    type Assert8 = AssertGreater<U2, U1, ()>;
    type Assert9 = AssertGreaterOrEqual<U2, U1, ()>;
    type Assert11 = AssertNotEqual<U1, U2, ()>;

    type Assert12 = IfElsePredicate<True, ((),), ()>;
    type Assert13 = IfElsePredicate<False, ((),), ()>;

    type Assert14 = IfElseNotPredicate<False, ((),), ()>;
    type Assert15 = IfElseNotPredicate<True, ((),), ()>;

    type Assert16 = IfElseEqual<U1, U1, ((),), ()>;
    type Assert17 = IfElseEqual<U1, U2, ((),), ()>;

    type Assert18 = IfElseLess<U1, U2, ((),), ()>;
    type Assert19 = IfElseLess<U1, U1, ((),), ()>;
    type Assert20 = IfElseLess<U2, U1, ((),), ()>;

    type Assert21 = IfElseLessOrEqual<U1, U2, ((),), ()>;
    type Assert22 = IfElseLessOrEqual<U1, U1, ((),), ()>;
    type Assert23 = IfElseLessOrEqual<U2, U1, ((),), ()>;

    type Assert27 = IfElseGreater<U1, U2, ((),), ()>;
    type Assert28 = IfElseGreater<U1, U1, ((),), ()>;
    type Assert29 = IfElseGreater<U2, U1, ((),), ()>;

    type Assert33 = IfElseGreaterOrEqual<U1, U2, ((),), ()>;
    type Assert34 = IfElseGreaterOrEqual<U1, U1, ((),), ()>;
    type Assert35 = IfElseGreaterOrEqual<U2, U1, ((),), ()>;

    type Assert36 = IfElseNotEqual<U1, U2, ((),), ()>;
    type Assert37 = IfElseNotEqual<U1, U1, ((),), ()>;
    type Assert38 = IfElseNotEqual<U2, U1, ((),), ()>;

    #[test]
    fn control_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert8 = ();
        let _: Assert9 = ();
        let _: Assert11 = ();

        let _: Assert12 = ((),);
        let _: Assert13 = ();

        let _: Assert14 = ((),);
        let _: Assert15 = ();

        let _: Assert16 = ((),);
        let _: Assert17 = ();

        let _: Assert18 = ((),);
        let _: Assert19 = ();
        let _: Assert20 = ();

        let _: Assert21 = ((),);
        let _: Assert22 = ((),);
        let _: Assert23 = ();

        let _: Assert27 = ();
        let _: Assert28 = ();
        let _: Assert29 = ((),);

        let _: Assert33 = ();
        let _: Assert34 = ((),);
        let _: Assert35 = ((),);

        let _: Assert36 = ((),);
        let _: Assert37 = ();
        let _: Assert38 = ((),);
    }
}
