use std::marker::PhantomData;

// maybe def

/// A trait analogous to [Option](std::option::Option).
pub trait Maybe {}

// just def

pub struct Just<T> {
    _phantom: PhantomData<T>,
}

impl<T> Maybe for Just<T> {}

// nothing def

pub struct Nothing;

impl Maybe for Nothing {}

// unwrap op

/// A type operator that unwraps [Just<T>](Just).
pub trait Unwrap
where
    Self: Maybe,
{
    type Output;
}

pub type UnwrapOutput<T> = <T as Unwrap>::Output;

impl<T> Unwrap for Just<T> {
    type Output = T;
}

// unwrap or default op

/// A type operator that unwraps [Just<T>](Just),
/// or derives to default type for [Nothing].
pub trait UnwrapOr<DefaultValue>
where
    Self: Maybe,
{
    type Output;
}

pub type UnwrapOrOutput<T, DefaultValue> = <T as UnwrapOr<DefaultValue>>::Output;

impl<T, DefaultValue> UnwrapOr<DefaultValue> for Just<T> {
    type Output = T;
}

impl<DefaultValue> UnwrapOr<DefaultValue> for Nothing {
    type Output = DefaultValue;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::IfSameOutput;
    use typenum::consts::*;

    type Opt1 = Just<U3>;
    type Opt2 = Nothing;

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert1 = AssertSame<UnwrapOutput<Opt1>, U3>;
    type Assert2 = AssertSame<UnwrapOrOutput<Opt1, U0>, U3>;
    type Assert3 = AssertSame<UnwrapOrOutput<Opt2, U0>, U0>;

    #[test]
    fn maybe_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
    }
}
