//! Numeric type operators and functors.

use std::ops::{Add, Sub};
use typenum::{Add1, Bit, Sub1, UInt, UTerm, Unsigned, B0, B1};

pub mod ops {
    use super::*;

    // popcount

    pub trait PopCount
    where
        Self: Unsigned,
        Self::Output: Unsigned,
    {
        type Output;
    }

    impl<Input> PopCount for Input
    where
        Input: Unsigned + PopCountWithBase<UTerm>,
    {
        type Output = op_aliases::PopCountWithBase<Input, UTerm>;
    }

    // popcount with base

    pub trait PopCountWithBase<Base>
    where
        Self: Unsigned,
        Self::Output: Unsigned,
        Base: Unsigned,
    {
        type Output;
    }

    impl<Base> PopCountWithBase<Base> for UTerm
    where
        Base: Unsigned,
    {
        type Output = Base;
    }

    impl<Base, U> PopCountWithBase<Base> for UInt<U, B0>
    where
        U: Unsigned + PopCountWithBase<Base>,
        Base: Unsigned,
    {
        type Output = op_aliases::PopCountWithBase<U, Base>;
    }

    impl<Base, U> PopCountWithBase<Base> for UInt<U, B1>
    where
        U: Unsigned + PopCountWithBase<Add1<Base>>,
        Base: Unsigned + Add<B1>,
        Add1<Base>: Unsigned,
    {
        type Output = op_aliases::PopCountWithBase<U, Add1<Base>>;
    }

    // binary to sequence of bits

    pub trait BitSeqOf
    where
        Self: Unsigned,
        Self::Output: Unsigned,
    {
        type Output;
    }

    impl<Input> BitSeqOf for Input
    where
        Input: Unsigned + BitSeqOfWithBase<UTerm>,
    {
        type Output = op_aliases::BitSeqOfWithBase<Input, UTerm>;
    }

    // binary to sequence of bits with base

    pub trait BitSeqOfWithBase<Base>
    where
        Self: Unsigned,
        Self::Output: Unsigned,
        Base: Unsigned,
    {
        type Output;
    }

    impl<Base> BitSeqOfWithBase<Base> for UTerm
    where
        Base: Unsigned,
    {
        type Output = Base;
    }

    impl<Base> BitSeqOfWithBase<Base> for UInt<UTerm, B1>
    where
        Base: Unsigned + Add<B1>,
    {
        type Output = op_aliases::BitSeqOfWithBase<UTerm, UInt<Base, B1>>;
    }

    impl<Base, U, B> BitSeqOfWithBase<Base> for UInt<UInt<U, B>, B1>
    where
        UInt<UInt<U, B>, B0>: BitSeqOfWithBase<UInt<Base, B1>>,
        Base: Unsigned,
        U: Unsigned,
        B: Bit,
    {
        type Output = op_aliases::BitSeqOfWithBase<UInt<UInt<U, B>, B0>, UInt<Base, B1>>;
    }

    impl<Base, U> BitSeqOfWithBase<Base> for UInt<U, B0>
    where
        Base: Unsigned,
        U: Unsigned + Sub<B1>,
        UInt<Sub1<U>, B1>: BitSeqOfWithBase<UInt<Base, B1>>,
    {
        type Output = op_aliases::BitSeqOfWithBase<UInt<Sub1<U>, B1>, UInt<Base, B1>>;
    }

}

pub mod op_aliases {
    use super::*;

    pub type PopCount<Input> = <Input as ops::PopCount>::Output;
    pub type PopCountWithBase<Input, Base> = <Input as ops::PopCountWithBase<Base>>::Output;
    pub type BitSeqOf<Input> = <Input as ops::BitSeqOf>::Output;
    pub type BitSeqOfWithBase<Input, Base> = <Input as ops::BitSeqOfWithBase<Base>>::Output;
}

#[cfg(test)]
mod tests {
    use super::op_aliases::*;
    use crate::control::op_aliases::IfSame;
    use typenum::{U0, U1, U15, U2, U3, U4, U7};

    type Assert1 = IfSame<(), PopCount<U0>, U0>;
    type Assert2 = IfSame<(), PopCount<U1>, U1>;
    type Assert3 = IfSame<(), PopCount<U2>, U1>;
    type Assert4 = IfSame<(), PopCount<U3>, U2>;
    type Assert5 = IfSame<(), BitSeqOf<U0>, U0>;
    type Assert6 = IfSame<(), BitSeqOf<U1>, U1>;
    type Assert7 = IfSame<(), BitSeqOf<U2>, U3>;
    type Assert8 = IfSame<(), BitSeqOf<U3>, U7>;
    type Assert9 = IfSame<(), BitSeqOf<U4>, U15>;

    #[test]
    fn numeric_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5 = ();
        let _: Assert6 = ();
        let _: Assert7 = ();
        let _: Assert8 = ();
        let _: Assert9 = ();
    }
}
