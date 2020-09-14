use crate::{common::*, functional::Func};

pub use base::*;
pub use ops::*;

mod base {
    // maybe def

    /// A trait analogous to [Option](std::option::Option).
    pub trait Maybe {}

    // just def

    /// A type analogous to `Some`.
    pub struct Just<T>(T);

    impl<T> Maybe for Just<T> {}

    // nothing def

    /// A type analogous to `None`.
    pub struct Nothing;

    impl Maybe for Nothing {}
}

mod ops {
    use super::*;

    typ! {
        pub fn Unwrap<value>(Just::<value>: Maybe) {
            value
        }

        pub fn UnwrapOr<maybe, default>(maybe: Maybe, default: _) {
            match maybe {
                #[generics(value)]
                Just::<value> => value,
                Nothing => default,
            }
        }

        pub fn IsJust<maybe>(maybe: Maybe) -> Bit {
            match maybe {
                #[generics(value)]
                Just::<value> => true,
                Nothing => false,
            }
        }

        pub fn IsNothing<maybe>(maybe: Maybe) -> Bit {
            match maybe {
                #[generics(value)]
                Just::<value> => false,
                Nothing => true,
            }
        }

        pub fn Map<maybe, func>(maybe: Maybe, func: _) -> Maybe {
            match maybe {
                #[generics(value)]
                Just::<value> => {
                    let new_value = <func as Func<value>>::Output;
                    Just::<new_value>
                }
                Nothing => Nothing
            }
        }
    }
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::{
    //     boolean::Boolean,
    //     control::{IfElsePredicate, IfElsePredicateOutput, IfSameOutput},
    //     functional::{Applicative, FMap},
    //     numeric::AddOneMap,
    // };
    // use typenum::{consts::*, GrEq, IsGreaterOrEqual, Unsigned};

    // // unwrap
    // type Opt1 = Just<U3>;
    // type Opt2 = Nothing;

    // type SameOp<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    // type Assert1 = SameOp<Unwrap<Opt1>, U3>;
    // type Assert2 = SameOp<UnwrapOr<Opt1, U0>, U3>;
    // type Assert3 = SameOp<UnwrapOr<Opt2, U0>, U0>;

    // // map
    // struct BoxFunc;

    // impl<Input> Map<Input> for BoxFunc {
    //     type Output = Box<Input>;
    // }

    // type Assert4 = IfSameOutput<(), MaybeMap<Just<i8>, BoxFunc>, Just<Box<i8>>>;
    // type Assert5 = IfSameOutput<(), MaybeMap<Nothing, BoxFunc>, Nothing>;

    // // filter
    // struct ThresholdFunc;

    // impl<Input> Map<Input> for ThresholdFunc
    // where
    //     Input: Unsigned + IsGreaterOrEqual<U4>,
    //     GrEq<Input, U4>: Boolean,
    //     Just<Input>: IfElsePredicate<GrEq<Input, U4>, Nothing>,
    // {
    //     type Output = IfElsePredicateOutput<Just<Input>, GrEq<Input, U4>, Nothing>;
    // }

    // type Assert6 = IfSameOutput<(), MaybeFilter<Just<U8>, ThresholdFunc>, Just<U8>>;
    // type Assert7 = IfSameOutput<(), MaybeFilter<Just<U2>, ThresholdFunc>, Nothing>;
    // type Assert8 = IfSameOutput<(), MaybeFilter<Nothing, ThresholdFunc>, Nothing>;

    // // FMap
    // type Assert9 = IfSameOutput<(), FMap<Nothing, AddOneMap>, Nothing>;
    // type Assert10 = IfSameOutput<(), FMap<Just<U8>, AddOneMap>, Just<U9>>;

    // // Applicative
    // type Assert11 = IfSameOutput<(), Applicative<Nothing, Nothing>, Nothing>;
    // type Assert12 = IfSameOutput<(), Applicative<Just<AddOneMap>, Nothing>, Nothing>;
    // type Assert13 = IfSameOutput<(), Applicative<Nothing, Just<U7>>, Nothing>;
    // type Assert14 = IfSameOutput<(), Applicative<Just<AddOneMap>, Just<U7>>, Just<U8>>;

    #[test]
    fn maybe_test() {}
}
