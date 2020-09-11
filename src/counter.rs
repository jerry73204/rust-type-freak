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
use std::ops::{Add, Sub};
use typenum::{Add1, Bit, Sub1, UInt, UTerm, Unsigned, B0, B1};

pub mod base {
    use super::*;

    pub trait Counter
    where
        Self: List,
    {
    }

    pub type Step<Tail> = Cons<(), Tail>;

    impl<Tail> Counter for Step<Tail> where Tail: Counter {}

    impl Counter for Nil {}
}

pub use base::*;

pub mod ops {
    use super::*;

    // counter to unsigned

    pub trait ToUnsigned
    where
        Self: Counter,
        Self::Output: Unsigned,
    {
        type Output;
    }

    impl ToUnsigned for Nil {
        type Output = UTerm;
    }

    impl<Next> ToUnsigned for Step<Next>
    where
        Next: Counter + ToUnsigned,
        op_aliases::ToUnsigned<Next>: Add<B1>,
        Add1<op_aliases::ToUnsigned<Next>>: Unsigned,
    {
        type Output = Add1<op_aliases::ToUnsigned<Next>>;
    }

    // unsigned to counter

    pub trait ToCounter
    where
        Self: Unsigned,
        Self::Output: Counter,
    {
        type Output;
    }

    impl ToCounter for UTerm {
        type Output = Nil;
    }

    impl ToCounter for UInt<UTerm, B1> {
        type Output = Step<Nil>;
    }

    impl<U, B> ToCounter for UInt<UInt<U, B>, B1>
    where
        U: Unsigned,
        B: Bit,
        UInt<UInt<U, B>, B0>: ToCounter,
    {
        type Output = Step<op_aliases::ToCounter<UInt<UInt<U, B>, B0>>>;
    }

    impl<U> ToCounter for UInt<U, B0>
    where
        U: Unsigned + Sub<B1>,
        UInt<Sub1<U>, B1>: ToCounter,
    {
        type Output = Step<op_aliases::ToCounter<UInt<Sub1<U>, B1>>>;
    }
}

pub mod op_aliases {
    use super::*;

    pub type ToUnsigned<Count> = <Count as ops::ToUnsigned>::Output;
    pub type ToCounter<Value> = <Value as ops::ToCounter>::Output;
}

pub mod macros {
    #[macro_export]
    macro_rules! Counter {
        ($size:ty) => { $crate::counter::op_aliases::ToCounter<$size> };
    }
}

#[cfg(test)]
mod tests {
    use super::{op_aliases::*, *};
    use crate::{control::op_aliases::*, Counter};
    use typenum::{U0, U1, U2, U3};

    type Assert1 = AssertSame<Counter![U0], Nil, ()>;
    type Assert2 = AssertSame<Counter![U1], Step<Nil>, ()>;
    type Assert3 = AssertSame<Counter![U2], Step<Step<Nil>>, ()>;
    type Assert4 = AssertSame<Counter![U3], Step<Step<Step<Nil>>>, ()>;
    type Assert5 = AssertSame<ToUnsigned<Nil>, U0, ()>;
    type Assert6 = AssertSame<ToUnsigned<Step<Nil>>, U1, ()>;
    type Assert7 = AssertSame<ToUnsigned<Step<Step<Nil>>>, U2, ()>;
    type Assert8 = AssertSame<ToUnsigned<Step<Step<Step<Nil>>>>, U3, ()>;

    #[test]
    fn counter_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
    }
}
